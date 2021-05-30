use crate::Pool;
use diesel::RunQueryDsl;
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use std::sync::Arc;

use crate::models::user::{User, NewUser, CreateUserInput};

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
}

impl juniper::Context for Context {}

pub struct Query;

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

pub struct  Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(context: &Context, user: CreateUserInput,) -> FieldResult<User>  {
        use crate::schema::users;

        let user = NewUser{
            name: &user.name,
            email: &user.email,
            password: &user.password,
        };
        let conn = context.pool.get().unwrap();

       let ress = diesel::insert_into(users::table)
       .values(&user).get_result(&conn);

       match ress {
           Ok(t) => Ok(t),
           Err(e) => FieldResult::Err(FieldError::from(e))
       }

    }
}


pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query,
        Mutation,
        EmptySubscription::new(),
    )
}
