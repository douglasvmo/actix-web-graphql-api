pub mod model;
use actix_web::web;
pub mod handler;

pub(super) fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(handler::graphql_playground)))
        .service(web::resource("/graphql").route(web::post().to(handler::graphql)));
}
