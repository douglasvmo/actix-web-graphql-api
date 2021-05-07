mod graphql;

use crate::Pool;
use actix_web::{web, HttpResponse};
use graphql::{create_schema, Context, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    let schema = create_schema();
    config
        .data(schema)
        .service(web::resource("/").route(web::get().to(health)))
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)));
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charse=utf-8")
        .body(html)
}

async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<Pool>,
) -> HttpResponse {
    let pool = pool.into_inner();
    let context = Context { pool };
    let res = data.execute(&schema, &context).await;

    HttpResponse::Ok().json(res)
}
