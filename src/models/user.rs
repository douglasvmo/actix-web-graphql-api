use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
}

#[derive(GraphQLInputObject)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
    active: bool,
}
