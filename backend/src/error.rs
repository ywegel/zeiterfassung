use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::repositories::region_repositories::RepositoryError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Error while accessing data")]
    RepositoryError(#[from] RepositoryError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::RepositoryError(repository_error) => match repository_error {
                RepositoryError::TimerNotRunning => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    repository_error.to_string(),
                ),
                RepositoryError::DatabaseError(ref e) => {
                    eprintln!("{}", e);

                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            },
        }
        .into_response()
    }
}
