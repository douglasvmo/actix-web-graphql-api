use crate::database;
use crate::database::PoolConnection;
use dotenv::dotenv;

pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub url: String,
    pub secret_key: String,
    pub database_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
        let secret_key = std::env::var("SECRET_KEY").expect("set SECRET_KEY");
        let url = std::env::var("URL").expect("set URL");
        let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = std::env::var("PORT").unwrap().parse::<i32>().unwrap();

        Self {
            url,
            database_url,
            secret_key,
            host,
            port,
        }
    }

    pub fn configured_pool(&self) -> PoolConnection {
        database::init_pool(self.database_url.clone())
    }
}
