pub mod model;

use crate::errors::ServiceResult;
use crate::graphql::model::{Context, Mutation, Query};
use crate::jwt::create_token;
use model::{InsertableUser, User, UserAuth};

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct UserLogin {
    pub login: String,
    pub password: String,
}

#[juniper::graphql_object(context = Context)]
impl Query {
    fn users(context: &Context) -> ServiceResult<Vec<User>> {
        let conn = context.get_conn()?;
        Ok(User::get_users(&conn)?)
    }

    fn whoami(context: &Context) -> ServiceResult<User> {
        let auth = context.token.get_auth()?;
        let conn = context.get_conn()?;
        Ok(User::get_by_id(&auth.id, &conn)?)
    }
}

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub fn register_user(context: &Context, user: InsertableUser) -> ServiceResult<User> {
        let conn = context.get_conn()?;
        Ok(User::new(user, &conn)?)
    }

    pub fn token(context: &Context, user_login: UserLogin) -> ServiceResult<String> {
        let UserLogin { login, password } = user_login;
        let conn = context.get_conn()?;
        let user = User::get_by_login(login, &conn)?;
        let auth_user = user.check_password(password)?;
        UserAuth::new_login(user.id, &conn)?;
        Ok(create_token(auth_user)?)
    }
}
