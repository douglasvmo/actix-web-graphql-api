#[macro_use]
extern crate diesel;
extern crate dotenv;
mod database;
mod errors;
mod graphql;
mod jwt;
mod schema;
mod users;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    //create db connection pool
    let pool = database::init_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
