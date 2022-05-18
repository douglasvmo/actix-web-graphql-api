use std::fmt::Debug;

use crate::errors::{ServiceError, ServiceResult};
use crate::models::auth::Auth;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
pub mod handler;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: i64,
    pub id: uuid::Uuid,
}

impl Claims {
    pub fn new(id: uuid::Uuid, auth_duration_in_hour: u16) -> Self {
        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Self {
            exp: exp.timestamp(),
            id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthorizationService {
    jwt: Option<Claims>,
}

impl AuthorizationService {
    pub fn new(token: Option<&str>) -> Self {
        match token {
            None => Self { jwt: None },
            Some(token) => match decode_token(token) {
                Err(_) => Self { jwt: None },
                Ok(claims) => Self { jwt: Some(claims) },
            },
        }
    }

    pub(crate) fn token_to(login: &Auth) -> ServiceResult<String> {
        let Auth { user_id, .. } = *login;
        let claims = Claims::new(user_id, 1);
        create_token(claims)
    }

    pub fn get_claims(&self) -> ServiceResult<Claims> {
        match self.jwt.clone() {
            None => Err(ServiceError::Unauthorized),
            Some(jwt) => Ok(jwt),
        }
    }
}

pub fn decode_token(token: &str) -> ServiceResult<Claims> {
    let key = DecodingKey::from_secret("cat".as_ref());
    let val = Validation::new(jsonwebtoken::Algorithm::HS512);
    decode::<Claims>(&token, &key, &val)
        .map(|data| data.claims)
        .map_err(|_| ServiceError::Unauthorized)
}

pub(crate) fn create_token(claims: Claims) -> Result<String, ServiceError> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    let key = jsonwebtoken::EncodingKey::from_secret("cat".as_ref());

    jsonwebtoken::encode(&header, &claims, &key)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
