use crate::database::get_conn;
use crate::models::auth::{Auth, NewAuth};
use crate::schema::auths;
use crate::{database::PoolConnection, errors::ServiceResult};
use diesel::prelude::*;
use std::sync::Arc;

pub struct AuthRepository {
    pool: Arc<PoolConnection>,
}

impl AuthRepository {
    pub fn new(pool: Arc<PoolConnection>) -> Self {
        Self { pool }
    }

    pub fn create(&self, user_id: uuid::Uuid) -> ServiceResult<Auth> {
        let conn = get_conn(&self.pool)?;
        Ok(diesel::insert_into(auths::dsl::auths)
            .values(NewAuth::new_login(user_id))
            .get_result(&conn)?)
    }
}
