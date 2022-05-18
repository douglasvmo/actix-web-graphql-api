use super::decode_token;
use super::AuthorizationService;
use actix_web::{dev::Payload, http::header, Error, FromRequest, HttpRequest};
use juniper::futures;

lazy_static::lazy_static! {
    static ref BEARER_REGEXP: regex::Regex = regex::Regex::new(r"^Bearer\s(.*)$").expect("Bearer regexp failed!");
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|authorization| {
                BEARER_REGEXP
                    .captures(authorization)
                    .and_then(|cap| cap.get(1))
            })
            .map(|v| v.as_str());

        futures::future::ready(Ok(Self::new(token)))
    }
}
