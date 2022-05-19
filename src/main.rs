#[macro_use]
extern crate diesel;
extern crate dotenv;
mod config;
mod database;
mod errors;
mod graphql;
mod jwt;
mod models;
mod repositories;
mod schema;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::ServerConfig::from_env();
    let pool = config.configured_pool();
    let server_addr = format!("{}:{}", config.host, config.port);
    
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql::routes)
    })
    .bind(server_addr)?
    .run()
    .await
}
