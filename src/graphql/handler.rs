use crate::graphql::model::{Context, Schema};
use crate::database::Pool;
use actix_web::{web, HttpResponse};
use juniper::http::{playground, GraphQLRequest};
use crate::jwt::model::BearerToken;

pub(super) async fn graphql_playground() -> HttpResponse {
    let html = playground::playground_source("/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charse=utf-8")
        .body(html)
}

pub(super) async fn graphql(
    token: BearerToken,
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<Pool>,
) -> HttpResponse {
    let pool = pool.into_inner();
    let context = Context { pool, token };
    let res = data.execute(&schema, &context).await;

    HttpResponse::Ok().json(res)
}
