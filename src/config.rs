use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
    pub server: Server,
    pub jwt: Jwt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expires_in: String,
    pub maxage: i64,
}

fn evar(key: &str) -> Result<String, env::VarError> {
    env::var(format!("ELNAFO_{}", key))
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_env(&mut self) -> Result<&Self, ConfigError> {
        dotenv().ok();

        self.database.host = evar("DATABASE_HOST").unwrap_or(self.database.host.to_owned());
        self.database.port = evar("DATABASE_PORT")
            .unwrap_or(self.database.port.to_string())
            .parse()?;
        self.database.user = evar("DATABASE_USER").unwrap_or(self.database.user.to_owned());
        self.database.password =
            evar("DATABASE_PASSWORD").unwrap_or(self.database.password.to_owned());
        self.database.name = evar("DATABASE_NAME").unwrap_or(self.database.name.to_owned());

        Ok(self)
    }

    pub fn open(path: &std::path::Path) -> Result<Config, ConfigError> {
        fs::read_to_string(path)?.parse()
    }

    pub fn data_dir() -> Result<std::path::PathBuf, ConfigError> {
        let cwd = std::env::current_dir()?;
        if cfg!(debug_assertions) {
            Ok(cwd.join("temp"))
        } else {
            Ok(cwd)
        }
    }

    pub fn to_string(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string(self)?)
    }

    pub fn write(&self, path: &std::path::Path) -> Result<(), ConfigError> {
        Ok(fs::write(path, self.to_string()?)?)
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.name
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database: Database {
                host: String::from("localhost"),
                port: 5432,
                user: String::from("elnafo"),
                password: String::from("test"),
                name: String::from("elnafo"),
            },
            server: Server {
                address: String::from("127.0.0.1"),
                port: 54600,
            },
            jwt: Jwt {
                secret: String::from("change_this_secret"),
                expires_in: String::from("60m"),
                maxage: 3600,
            },
        }
    }
}

impl std::str::FromStr for Config {
    type Err = ConfigError;
    fn from_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|_| ConfigError::Parse)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Parse,
    StringParse,
    Serialize,
    IO,
}

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Parse => write!(f, "Failed to parse Config from string"),
            Self::StringParse => write!(f, "Failed to parse environment variable"),
            Self::Serialize => write!(f, "Failed to serialize Config to TOML"),
            Self::IO => write!(f, "Faild to write file"),
        }
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(_: toml::ser::Error) -> Self {
        ConfigError::Serialize
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(_: std::io::Error) -> Self {
        ConfigError::IO
    }
}

impl From<std::num::ParseIntError> for ConfigError {
    fn from(_: std::num::ParseIntError) -> Self {
        ConfigError::StringParse
    }
}
