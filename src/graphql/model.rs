use crate::database::PoolConnection;
use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::AuthorizationService;
use crate::models::user::{NewUser, User};
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use juniper::Context as JuniperContext;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Context {
    pub pool: Arc<PoolConnection>,
    pub auth: AuthorizationService,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(pool: Arc<PoolConnection>, auth: AuthorizationService) -> Self {
        Self { auth, pool }
    }
    pub fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }
    pub fn auth_repository(&self) -> AuthRepository {
        AuthRepository::new(self.pool.clone())
    }
}

pub(crate) struct Query;
pub(crate) struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Query {
    pub fn users(context: &Context) -> ServiceResult<Vec<User>> {
        context.user_repository().all()
    }
    pub fn whoami(context: &Context) -> ServiceResult<User> {
        context
            .auth
            .get_claims()
            .and_then(|claims| context.user_repository().find(claims.id))
    }
}

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub fn register_user(context: &Context, input: NewUser) -> ServiceResult<User> {
        context.user_repository().create(input)
    }
    pub fn token(context: &Context, login: String, password: String) -> ServiceResult<String> {
        let token = context
            .user_repository()
            .find_by_login(login)
            .and_then(|usr| match usr.verify_password(password) {
                false => Err(ServiceError::Unauthorized),
                true => Ok(usr),
            })
            .and_then(|usr| context.auth_repository().create(usr.id))
            .and_then(|login| AuthorizationService::token_to(&login))?;

        Ok(token)
    }
}

pub(crate) type Schema =
    juniper::RootNode<'static, Query, Mutation, juniper::EmptySubscription<Context>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new())
}
