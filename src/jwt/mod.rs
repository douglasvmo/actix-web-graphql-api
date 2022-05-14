use futures::channel::mpsc::channel;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::errors::ServiceError;
use crate::users::model::User;

use self::model::Claims;

pub mod handler;
pub mod model;

pub(crate) fn decode_token(token: &str) -> Result<model::Claims, ServiceError> {
    let key = DecodingKey::from_secret("cat".as_ref());
    let val = Validation::new(jsonwebtoken::Algorithm::HS512);
    decode::<model::Claims>(token, &key, &val)
        .map(|data| data.claims)
        .map_err(|e| ServiceError::BadRequest)
}

pub(crate) fn create_token(user: &User) -> Result<String, ServiceError> {
    let User { id, .. } = user;
    let claims = Claims::new(id, 1);
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    let key = jsonwebtoken::EncodingKey::from_secret("cat".as_ref());

    jsonwebtoken::encode(&header, &claims, &key).map_err(|e| ServiceError::BadRequest)
}
