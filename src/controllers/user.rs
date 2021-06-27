use crate::controllers::schema::{Context, Mutation, Query};
use crate::models::user::*;
use diesel::RunQueryDsl;
use juniper::{FieldError, FieldResult};

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn api_version() -> &str {
        "1.0"
    }
    async fn users(context: &Context) -> Vec<User> {
        let conn = context.pool.get().unwrap();
        User::all(&conn)
    }
}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(context: &Context, user: CreateUserInput) -> FieldResult<User> {
        use crate::schema::users;

        let user = NewUser {
            name: &user.name,
            email: &user.email,
            password: &user.password,
        };
        let conn = context.pool.get().unwrap();

        let ress = diesel::insert_into(users::table)
            .values(&user)
            .get_result(&conn);

        match ress {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }
}
