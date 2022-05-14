use juniper::{Context as JuniperContext, EmptySubscription};

use crate::database::{get_conn, PoolConnection};
use crate::errors::ServiceResult;
use crate::jwt::model::DecodedToken;
use crate::users::model::{InsertableUser, User};

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
}

pub(crate) struct Query;
pub(crate) struct Mutation;

pub(crate) type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

#[juniper::graphql_object(context = Context)]
impl Query {
    fn users(context: &Context) -> ServiceResult<Vec<User>> {
        let conn = get_conn(&context.db)?;
        Ok(User::get_users(&conn)?)
    }
}

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub fn register_user(context: &Context, user: InsertableUser) -> ServiceResult<User> {
        let conn = get_conn(&context.db)?;
        Ok(User::register(user, &conn)?)
    }
}
