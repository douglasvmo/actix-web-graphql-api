use crate::database::{Connection, PoolConnection};
use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::model::DecodedToken;
use juniper::{Context as JuniperContext, EmptySubscription};

#[derive(Clone)]
pub(crate) struct Context {
    pub db: PoolConnection,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(db: PoolConnection, token: DecodedToken) -> Self {
        Self { token, db }
    }
    pub fn get_conn(&self) -> ServiceResult<Connection> {
        Ok(self
            .db
            .get()
            .map_err(|_| ServiceError::UnableToConnectToDb)?)
    }
}

pub(crate) struct Query;
pub(crate) struct Mutation;

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
