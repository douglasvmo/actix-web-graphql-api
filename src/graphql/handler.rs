use std::sync::Arc;

use crate::database::PoolConnection;
use crate::graphql::model::{Context, Schema};
use crate::jwt::model::DecodedToken;
use actix_web::{web, HttpResponse};
use juniper::http::{playground, GraphQLRequest};

pub(super) async fn graphql_playground() -> HttpResponse {
    let html = playground::playground_source("/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charse=utf-8")
        .body(html)
}

pub(super) async fn graphql(
    token: DecodedToken,
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Arc<Schema>>,
    pool: web::Data<PoolConnection>,
) -> HttpResponse {
    let context = Context::new(pool.as_ref().to_owned(), token);

    let res = data.execute_sync(&schema, &context);

    match res.is_ok() {
        true => HttpResponse::Ok(),
        false => HttpResponse::BadRequest(),
    }
    .content_type("application/json")
    .json(res)
}
