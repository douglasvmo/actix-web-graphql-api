use std::fmt::Debug;
use std::sync::Arc;

use crate::database::PoolConnection;
use crate::graphql::model::{Context, Schema};
use crate::jwt::model::DecodedToken;
use actix_web::{web, Error, HttpResponse};
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
) -> Result<HttpResponse, Error> {
    let context = Context::new(pool.as_ref().to_owned(), token);
    let res = data.execute(&schema, &context).await;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(res))
}
