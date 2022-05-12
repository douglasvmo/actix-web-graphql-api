use actix_web::web::Data;
use juniper::{Context as JuniperContext, EmptySubscription};

use crate::errors::ServiceResult;
use crate::users::model::{InsertableUser, User};
use crate::{database::Pool, jwt::model::BearerToken};

#[derive(Clone)]
pub(crate) struct Context {
    pub pool: Data<Pool>,
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
    fn users(context: &Context) -> ServiceResult<Vec<User>> {
        let pool = context.pool.clone();
        Ok(User::get_users(pool)?)
    }
}

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub fn register_user(context: &Context, user: InsertableUser) -> ServiceResult<User> {
        let conn = context.pool.clone();
        Ok(User::register(user, conn)?)
    }
}
