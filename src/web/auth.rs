use actix_web::dev::Payload;
use actix_web::{error, FromRequest, HttpRequest};
use base64::Engine;
use std::future::{ready, Ready};

pub struct BasicAuth;

impl BasicAuth {
    pub fn expected_joined_value() -> String {
        std::env::var("SD_BASIC_AUTH").unwrap_or_else(|_| "admin:password".to_owned())
    }

    pub fn user_pass() -> anyhow::Result<(String, String)> {
        let expected = Self::expected_joined_value();

        let mut iter = expected.split(':');
        let username = iter.next().ok_or(anyhow::Error::msg(
            "Could not extract basic auth username from configuration",
        ))?;
        let password = iter.next().ok_or(anyhow::Error::msg(
            "Could not extract basic auth password from configuration",
        ))?;

        Ok((username.to_owned(), password.to_owned()))
    }

    pub fn authorize(token: &str) -> error::Result<Self> {
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(token)
            .map(String::from_utf8)
            .map_err(|_| {
                error::ErrorUnauthorized("Invalid basic authorization header base64 encoding")
            })?
            .map_err(|_| {
                error::ErrorUnauthorized(
                    "Invalid basic authorization header base64 value is not a utf8 string",
                )
            })?;

        if decoded == Self::expected_joined_value() {
            Ok(Self)
        } else {
            Err(error::ErrorUnauthorized(
                "Invalid basic authorization token",
            ))
        }
    }
}

impl TryFrom<&HttpRequest> for BasicAuth {
    type Error = error::Error;

    fn try_from(req: &HttpRequest) -> Result<Self, Self::Error> {
        let authorization = req
            .headers()
            .get("Authorization")
            .map(|m| m.to_str().map(|s| s.to_owned()))
            .ok_or(error::ErrorUnauthorized(
                "Basic authorization header expected",
            ))?
            .map_err(|_| error::ErrorUnauthorized("Invalid basic authorization header"))?;

        let token = authorization
            .trim()
            .split(' ')
            .last()
            .ok_or(error::ErrorUnauthorized(
                "Invalid basic authorization header",
            ))?;

        BasicAuth::authorize(token)
    }
}

impl FromRequest for BasicAuth {
    type Error = error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(req.try_into())
    }
}
