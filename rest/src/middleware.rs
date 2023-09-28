use actix_web::{
    dev::ServiceRequest,
    error::{self, Error},
    HttpMessage,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use hmac::{Hmac, Mac};
use jwt::{Claims, Header, Token, VerifyWithKey};
use minibar::auth::User;
use sha2::Sha384;
use std::env;

pub async fn authenticate(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret = env::var("JWT_SECRET").expect("missing JWT secret");
    let key = Hmac::<Sha384>::new_from_slice(secret.as_bytes()).unwrap();
    let token: Result<Token<Header, Claims, _>, _> = creds.token().verify_with_key(&key);

    match token.map_err(|err| match err {
        jwt::Error::InvalidSignature => error::ErrorUnauthorized(err),
        _ => error::ErrorBadRequest(err),
    }) {
        Ok(token) => {
            let claims = token.claims();
            req.extensions_mut().insert(claims.to_owned());
            req.extensions_mut().insert(User::from(
                claims.to_owned().registered.subject.unwrap_or_default(),
            ));
            Ok(req)
        }
        Err(err) => Err((err, req)),
    }
}
