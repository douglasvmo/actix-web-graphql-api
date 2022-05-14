use std::sync::Arc;
use actix_web::web;

pub mod model;
pub mod handler;

pub(super) fn routes(config: &mut web::ServiceConfig) {
    let schema = Arc::new(model::create_schema());
    config
        .data(schema)
        .service(web::resource("/").route(web::get().to(handler::graphql_playground)))
        .service(web::resource("/graphql").route(web::post().to(handler::graphql)));
}
