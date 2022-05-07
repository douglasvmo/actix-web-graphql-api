#[macro_use]
extern crate diesel;
extern crate dotenv;
mod database;
mod graphql_schema;
mod handlers;
mod models;
mod schema;


use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    //create db connection pool
    let pool = database::pool::init_pool();
    
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
