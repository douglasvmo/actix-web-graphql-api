use crate::graphql_schema::root::{Context, Mutation, Query};
use crate::models::user::*;
use chrono::prelude::{DateTime, Utc};
use juniper::{FieldError, FieldResult, GraphQLInputObject};

#[juniper::graphql_object]
impl User {
    fn id(&self) -> uuid::Uuid {
        self.id
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn email(&self) -> &str {
        self.email.as_str()
    }
    fn active(&self) -> bool {
        self.active
    }
    fn createAt(&self) -> String {
        let dt: DateTime<Utc> = self.created_at.clone().into();
        dt.format("%+").to_string()
    }
    fn updatedAt(&self) -> String {
        let dt: DateTime<Utc> = self.updated_at.clone().into();
        dt.format("%+").to_string()
    }
}

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn users(context: &Context) -> Vec<User> {
        let conn = context.pool.get().unwrap();
        User::get_all(&conn)
    }
}

#[derive(GraphQLInputObject)]
struct UserInput {
    name: String,
    email: String,
    cpf_cnpj: String,
    password: String,
}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let user = NewUser {
            name: &user.name,
            email: &user.email,
            cpf_cnpj: &user.cpf_cnpj, 
            password: &user.password,
        };
        let conn = context.pool.get().unwrap();

        let ress = User::insert(&conn, &user);

        match ress {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }
}
