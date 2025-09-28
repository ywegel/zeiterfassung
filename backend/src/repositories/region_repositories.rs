use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::models::region::Region;
use crate::models::region_history::RegionHistory;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("No timer is running for the region")]
    TimerNotRunning,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[async_trait]
pub trait RegionRepository: Send + Sync {
    async fn start_timer(&self, region: Region) -> Result<(), RepositoryError>;
    async fn stop_timer(&self, region: Region) -> Result<i64, RepositoryError>;
    async fn get_history(&self, region: Region) -> Result<Vec<RegionHistory>, RepositoryError>;
}

pub struct SqliteRegionRepository {
    pool: SqlitePool,
}

impl SqliteRegionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegionRepository for SqliteRegionRepository {
    async fn start_timer(&self, region: Region) -> Result<(), RepositoryError> {
        let now = Utc::now();

        // Stop any active timer
        sqlx::query(
            r#"
                UPDATE region_history
                SET stop_time = $1,
                    duration = (strftime('%s', $1) - strftime('%s', start_time))
                WHERE stop_time IS NULL
            "#,
        )
        .bind(now)
        .execute(&self.pool)
        .await?;

        // Start timer for this region
        sqlx::query(
            r#"
            INSERT INTO region_history (region, start_time)
            VALUES ($1, $2)
            "#,
        )
        .bind(&region)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn stop_timer(&self, region: Region) -> Result<i64, RepositoryError> {
        let now = Utc::now();
        let result: Option<(DateTime<Utc>,)> = sqlx::query_as(
            r#"
            UPDATE region_history
            SET stop_time = $1,
                duration = (strftime('%s', $1) - strftime('%s', start_time))
            WHERE region = $2 AND stop_time IS NULL
            RETURNING start_time
            "#,
        )
        .bind(now)
        .bind(&region)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some((start_time,)) => Ok(now.signed_duration_since(start_time).num_seconds()),
            None => Err(RepositoryError::TimerNotRunning),
        }
    }

    async fn get_history(&self, region: Region) -> Result<Vec<RegionHistory>, RepositoryError> {
        let result = sqlx::query_as::<_, RegionHistory>(
            r#"
            SELECT region, start_time, stop_time, duration
            FROM region_history
            WHERE region = $1
            ORDER BY start_time DESC
            "#,
        )
        .bind(&region)
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_start_timer(pool: SqlitePool) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);

        // When
        let result = repo.start_timer(Region::North).await;
        assert!(result.is_ok(), "Starting timer should succeed");

        // Then
        let history = repo
            .get_history(Region::North)
            .await
            .expect("History should succeed");
        assert_eq!(history.len(), 1, "History should contain one entry");
        assert_eq!(history[0].region, Region::North, "Region should be North");
        assert!(history[0].stop_time.is_none(), "Stop time should be None");
        assert!(history[0].duration.is_none(), "Duration should be None");

        Ok(())
    }

    #[sqlx::test]
    async fn test_start_while_timer_for_same_region_already_running(
        pool: SqlitePool,
    ) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);
        repo.start_timer(Region::North).await.unwrap();

        // When
        let result = repo.start_timer(Region::North).await;

        // Then
        assert!(result.is_ok(), "Starting the same timer should succeed");

        // The previous timer should have stopped
        let north_history = repo
            .get_history(Region::North)
            .await
            .expect("History should succeed");
        assert_eq!(
            north_history.len(),
            2,
            "North history should contain two entries, one is stopped and one started"
        );

        // The timers are sorted descending. The "first one" is the currently running
        // timer and the "second one" is the stopped timer
        assert!(
            north_history[0].stop_time.is_none(),
            "First timer should be running"
        );
        assert!(
            north_history[0].duration.is_none(),
            "First timer should not have a duration"
        );
        assert!(
            north_history[1].stop_time.is_some(),
            "Second timer should be stopped"
        );
        assert!(
            north_history[1].duration.is_some(),
            "Second timer should have a duration"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_start_while_other_timer_already_running(pool: SqlitePool) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);
        repo.start_timer(Region::North)
            .await
            .expect("First timer should start normally");

        // When
        let result = repo.start_timer(Region::South).await;
        assert!(result.is_ok(), "Starting another timer should succeed");

        // Then
        // Verify the previous timer (North) was stopped
        let north_history = repo
            .get_history(Region::North)
            .await
            .expect("History should succeed");
        assert_eq!(
            north_history.len(),
            1,
            "North history should contain one entry"
        );
        assert!(
            north_history[0].stop_time.is_some(),
            "North timer should be stopped"
        );
        assert!(
            north_history[0].duration.is_some(),
            "North timer should have a duration"
        );

        // Verify the new timer (South) is running
        let south_history = repo
            .get_history(Region::South)
            .await
            .expect("History should succeed");
        assert_eq!(
            south_history.len(),
            1,
            "South history should contain one entry"
        );
        assert!(
            south_history[0].stop_time.is_none(),
            "South timer should be running"
        );
        assert!(
            south_history[0].duration.is_none(),
            "South timer should have no duration"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_stop_timer(pool: SqlitePool) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);
        let start = Utc::now();
        repo.start_timer(Region::East)
            .await
            .expect("Starting timer should succeed");

        // We need to wait for more than 1s, as the duration calculated by sqlite is in
        // seconds
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

        // When
        let stop = Utc::now();
        let duration = repo
            .stop_timer(Region::East)
            .await
            .expect("Stopping timer should not fail");

        // Then
        assert!(duration >= 1, "Duration should be positiv");

        // Verify the timer was stopped
        let history = repo
            .get_history(Region::East)
            .await
            .expect("History should succeed");
        assert_eq!(history.len(), 1, "History should contain one entry");
        assert_eq!(history[0].region, Region::East, "Region should be East");
        assert!(history[0].stop_time.is_some(), "Stop time should be set");
        assert_eq!(history[0].duration, Some(duration), "Duration should match");

        Ok(())
    }

    #[sqlx::test]
    async fn test_stop_timer_not_running(pool: SqlitePool) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);

        // When
        let result = repo.stop_timer(Region::North).await;

        // Then
        assert!(
            matches!(result, Err(RepositoryError::TimerNotRunning)),
            "Should fail with NotRunning error"
        );

        // History should stay empty
        let history = repo
            .get_history(Region::West)
            .await
            .expect("History should succeed");
        assert!(history.is_empty(), "History should be empty");

        Ok(())
    }

    #[sqlx::test]
    async fn test_get_history_for_region(pool: SqlitePool) -> sqlx::Result<()> {
        // Given
        let repo = SqliteRegionRepository::new(pool);

        repo.start_timer(Region::West).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        repo.stop_timer(Region::West).await.unwrap();

        repo.start_timer(Region::West).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        repo.stop_timer(Region::West).await.unwrap();

        // When
        let history = repo
            .get_history(Region::West)
            .await
            .expect("Fetching the history should succeed");

        // Then
        assert_eq!(history.len(), 2, "History should contain two entries");
        assert_eq!(history[0].region, Region::West, "Region should be West");
        assert!(
            history[0].stop_time.is_some(),
            "First entry should be stopped"
        );
        assert!(
            history[0].duration.is_some(),
            "First entry should have duration"
        );
        assert!(
            history[0].start_time > history[1].start_time,
            "Entries should be ordered by start_time DESC"
        );

        Ok(())
    }
}
