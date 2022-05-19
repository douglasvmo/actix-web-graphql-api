use crate::database::{get_conn, PoolConnection};
use crate::errors::ServiceResult;
use crate::models::project::Project;
use crate::schema::projects;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use std::sync::Arc;

pub struct ProjectRepository {
    pool: Arc<PoolConnection>,
}

impl ProjectRepository {
    pub fn new(pool: Arc<PoolConnection>) -> Self {
        Self { pool }
    }
    pub fn all_to_owner(&self, user_id: uuid::Uuid) -> ServiceResult<Vec<Project>> {
        use self::projects::dsl::*;
        let conn = &get_conn(&self.pool)?;
        Ok(projects.filter(owner_id.eq(user_id)).load(conn)?)
    }
}
