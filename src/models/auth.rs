use crate::schema::auths;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
#[table_name = "auths"]
pub struct Auth {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub payload: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "auths"]
pub struct NewAuth {
    pub user_id: uuid::Uuid,
    pub payload: String,
}

impl NewAuth {
    pub fn new_login(user_id: uuid::Uuid) -> Self {
        Self {
            user_id,
            payload: "login".to_string(),
        }
    }
}
