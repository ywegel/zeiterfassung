use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub database_url: String,
    pub application_port: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error("Failed to load .env file: {0}")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Failed to deserialize environment variables: {0}")]
    Envy(#[from] envy::Error),
}

pub fn load_configuration(env_path: Option<&str>) -> Result<Configuration, ConfigurationError> {
    match env_path {
        None => dotenvy::dotenv()?,
        Some(path) => dotenvy::from_filename(path)?,
    };
    let config = envy::from_env()?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::ConfigurationError;
    use super::load_configuration;

    #[test]
    fn test_load_configuration_success() {
        let config = load_configuration(Some("./tests/resources/valid.env"))
            .expect("Failed to load valid configuration");
        assert_eq!(
            config.database_url,
            "postgres://user:pass@localhost:5432/db"
        );
        assert_eq!(config.application_port, 8080);
    }

    #[test]
    fn test_load_configuration_invalid_env() {
        let result = load_configuration(Some("./tests/resources/invalid.env"));
        assert!(matches!(result, Err(ConfigurationError::Envy(_))));
    }

    #[test]
    fn test_load_configuration_missing_file() {
        let result = load_configuration(Some("./tests/resources/nonexistent.env"));
        assert!(matches!(result, Err(ConfigurationError::Dotenv(_))));
    }
}
