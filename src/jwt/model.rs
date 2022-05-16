use crate::errors::{ServiceError, ServiceResult};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: i64,
    pub id: String,
}

impl Claims {
    pub fn new(id: &uuid::Uuid, auth_duration_in_hour: u16) -> Self {
        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Self {
            exp: exp.timestamp(),
            id: id.to_string(),
        }
    }
}

impl DecodedToken {
    pub fn get_auth(&self) -> ServiceResult<Claims> {
        match self.jwt.clone() {
            Some(jwt) => Ok(jwt),
            None => Err(ServiceError::Unauthorized),
        }
    }
}
