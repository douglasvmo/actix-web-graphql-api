use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    #[graphql(skip)]
    pub password: String,
    pub role_id: Option<i32>,
    pub active: bool,
    #[graphql(skip)]
    pub verification_payload: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, juniper::GraphQLInputObject)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    pub password: String,
}

pub fn hash_encode(password: String, salt: &str) -> String {
    use sha2::Digest;
    let mut sha512 = sha2::Sha512::new();
    sha512.update(password + salt);
    hex::encode(sha512.finalize())
}

impl User {
    pub fn verify_password(&self, password: String) -> bool {
        hash_encode(password, &self.email) == self.password
    }
}
