use actix_web::{http::header, dev::Payload, Error, FromRequest, HttpRequest};
use juniper::futures;
use super::model::BearerToken;


impl FromRequest for BearerToken {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token = req
        .headers()
        .get(header::AUTHORIZATION);

        futures::future::ready(Ok(BearerToken{jwt: None}))
    }
}