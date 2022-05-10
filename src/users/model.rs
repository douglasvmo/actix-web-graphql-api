use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    #[graphql(skip)]
    pub password: String,
    pub role_id: Option<i32>,
    pub active: bool,
    pub verification_code: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    pub password: String,
}
