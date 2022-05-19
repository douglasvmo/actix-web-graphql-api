use crate::schema::projects;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
#[table_name = "projects"]
pub struct Project {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
