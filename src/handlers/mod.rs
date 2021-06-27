use actix_web::{web, HttpResponse};
mod graphql;
use crate::graphql_schema;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    let schema = graphql_schema::root::create_schema();
    config
        .data(schema)
        .service(web::resource("/").route(web::get().to(health)))
        .service(
            web::resource("/graphql")
                .route(web::post().to(graphql::graphql))
                .route(web::get().to(graphql::graphql_playground)),
        );
}
