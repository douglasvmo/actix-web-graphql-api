use actix_web::{web, HttpResponse};
mod admin;
mod graphql;
use actix_identity::{CookieIdentityPolicy, IdentityService};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(health)))
        .service(
            web::resource("/graphql")
                .route(web::post().to(graphql::graphql))
                .route(web::get().to(graphql::graphql_playground)),
        )
        .service(
            web::resource("/admin")
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[0; 32])
                        .name("auth-cookie")
                        .secure(false),
                ))
                .route(web::get().to(admin::admin_page)),
        );
}
