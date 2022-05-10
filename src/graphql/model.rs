use juniper::{Context as JuniperContext, EmptySubscription} ;
use std::{sync::Arc};

use crate::{database::Pool, jwt::model::BearerToken};

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
    pub token: BearerToken,
}

impl JuniperContext for Context {}

pub(crate) struct Query;

pub(crate) struct Mutation;

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}


#[juniper::graphql_object(context = Context)]
impl Query {
    async fn hello() -> juniper::FieldResult<String> {
       Ok( "Hello Graphql".to_string())
    }
}

#[juniper::graphql_object(context = Context)]
impl Mutation {
   async fn create_hello(str: String) -> juniper::FieldResult<String>{
       Ok(str)
   }
}