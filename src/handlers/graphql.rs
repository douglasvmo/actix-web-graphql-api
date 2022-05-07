use crate::{graphql_schema::root::{Context, Schema}, database::Pool};
use actix_web::{web, HttpResponse};
use juniper::http::{playground, GraphQLRequest};

pub async fn graphql_playground() -> HttpResponse {
    let html = playground::playground_source("/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charse=utf-8")
        .body(html)
}

pub async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<Pool>,
) -> HttpResponse {
    let pool = pool.into_inner();
    let context = Context { pool };
    let res = data.execute(&schema, &context).await;

    HttpResponse::Ok().json(res)
}
