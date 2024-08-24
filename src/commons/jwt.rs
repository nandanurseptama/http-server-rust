use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, Mul},
    time::Duration,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim<T: Serialize> {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject of claim
    jti: String, // Jwt ID. Unique identifier; can be used to prevent the JWT from being replayed (allows a token to be used only once)
    data: T,     // optional jwt data
}

#[derive(Debug, Clone)]
pub struct JWT {
    auth_token_secret: String,
    refresh_token_secret: String,
}

impl JWT {
    pub fn new(auth_token_secret: String, refresh_token_secret: String) -> Self {
        Self {
            auth_token_secret,
            refresh_token_secret,
        }
    }
    fn encode<T: Serialize>(
        secret: String,
        claim: Claim<T>,
    ) -> jsonwebtoken::errors::Result<String> {
        return jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        );
    }

    pub fn generate_auth_token<T: Serialize>(
        self,
        data: T,
    ) -> jsonwebtoken::errors::Result<String> {
        let expire = Duration::from_secs(3600).mul(24).as_secs();
        let now = chrono::Utc::now().timestamp();
        let nbf = now;
        let iat = now;
        let exp = nbf.add(expire as i64);

        let jti = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let claim = Claim {
            aud: String::from("all"),
            data,
            iss: String::from("server"),
            exp: exp as usize,
            iat: iat as usize,
            jti,
            nbf: nbf as usize,
            sub: String::from("auth"),
        };
        return Self::encode(self.auth_token_secret, claim);
    }

    pub fn generate_refresh_token<T: Serialize>(
        self,
        data: T,
    ) -> jsonwebtoken::errors::Result<String> {
        let expire = Duration::from_secs(3600).mul(24).as_secs();
        let now = chrono::Utc::now().timestamp();
        let nbf = now.add(3600);
        let iat = now;
        let exp = nbf.add(expire as i64);

        let jti = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let claim = Claim {
            aud: String::from("all"),
            data,
            iss: String::from("server"),
            exp: exp as usize,
            iat: iat as usize,
            jti,
            nbf: nbf as usize,
            sub: String::from("refresh"),
        };
        return Self::encode(self.refresh_token_secret, claim);
    }
}
