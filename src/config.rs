use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub server: Server,
    pub jwt: Jwt,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub address: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
pub struct Jwt {
    pub secret: String,
    pub expires_in: String,
    pub maxage: i32,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            database: Database {
                host: env::var("DATABASE_HOST").unwrap_or("localhost".to_string()),
                port: env::var("DATABASE_PORT")
                    .unwrap_or("5432".to_string())
                    .parse()
                    .unwrap(),
                user: env::var("DATABASE_USER").unwrap_or("elnafo".to_string()),
                password: env::var("DATABASE_PASSWORD").unwrap_or("test".to_string()),
                name: env::var("DATABASE_NAME").unwrap_or("elnafo".to_string()),
            },
            server: Server {
                address: env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or("54600".to_string())
                    .parse()
                    .unwrap(),
            },
            jwt: Jwt {
                secret: env::var("JWT_SECRET").unwrap_or("change_this_secret".to_string()),
                expires_in: env::var("JWT_EXPIRES_IN").unwrap_or("60m".to_string()),
                maxage: env::var("JWT_MAXAGE")
                    .unwrap_or("60".to_string())
                    .parse()
                    .unwrap(),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}
