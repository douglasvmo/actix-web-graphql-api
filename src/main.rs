#[macro_use]
extern crate diesel;
extern crate dotenv;
mod graphql_schema;
mod handlers;
mod models;
mod schema;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    //create db connection pool
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let schema = Data::new(graphql_schema::root::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(schema.clone())
            .wrap(middleware::Logger::default())
            .configure(handlers::app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
