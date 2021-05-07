use crate::Pool;
use juniper::{EmptyMutation, RootNode};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
}

impl juniper::Context for Context {}

pub struct Query {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn apiVersion() -> &str {
        "1.0"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        EmptyMutation::<Context>::new(),
        EmptyMutation::<Context>::new(),
    )
}
