use crate::Pool;
use juniper::{EmptySubscription, RootNode};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
}

impl juniper::Context for Context {}

pub struct Query;

pub struct Mutation;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
