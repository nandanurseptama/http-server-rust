use jsonwebtoken::{DecodingKey, Validation};
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, Mul},
    time::Duration,
};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claim {
    aud: String,  // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject of claim
    jti: String, // Jwt ID. Unique identifier; can be used to prevent the JWT from being replayed (allows a token to be used only once)
    data: String, // optional jwt data
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
    fn encode(secret: String, claim: Claim) -> jsonwebtoken::errors::Result<String> {
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

        let data_string = serde_json::to_string(&data).map_err(|err| err)?;

        let jti = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let claim = Claim {
            aud: String::from("all"),
            data: data_string,
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
        let data_string = serde_json::to_string(&data).map_err(|err| err)?;

        let jti = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let claim = Claim {
            aud: String::from("all"),
            data: data_string,
            iss: String::from("server"),
            exp: exp as usize,
            iat: iat as usize,
            jti,
            nbf: nbf as usize,
            sub: String::from("refresh"),
        };
        return Self::encode(self.refresh_token_secret, claim);
    }

    fn decode(self, secret: &String, token: &String) -> Result<Claim, super::Error> {
        let validation = {
            let mut validation = Validation::default();
            validation.insecure_disable_signature_validation();
            validation.validate_exp = true;
            validation.validate_nbf = true;
            validation.validate_aud = false;
            validation
        };
        let token_data = jsonwebtoken::decode::<Claim>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map_err(|err| super::Error {
            cause: err.to_string(),
            message: String::from("failed to decode jwt token"),
            status: 500,
        })?;

        Ok(token_data.claims)
    }

    pub fn decode_auth_token<T: for<'a> Deserialize<'a>>(
        self,
        token: &String,
    ) -> Result<T, super::Error> {
        let claim = self.clone().decode(&self.auth_token_secret, token).map_err(|err| err)?;
        if claim.sub != "auth" {
            return Err(super::Error {
                cause: String::from("invalid auth token. token subject is not auth"),
                message: String::from("invalid auth token"),
                status: 400,
            });
        }

        let claim_data: &str = &claim.data;
        let data: T = serde_json::from_str(claim_data).map_err(|err| super::Error {
            cause: err.to_string(),
            message: String::from("failed to decode jwt token"),
            status: 500,
        })?;

        return Ok(data);
    }

    pub fn decode_refresh_token<T: for<'a> Deserialize<'a>>(
        self,
        token: &String,
    ) -> Result<T, super::Error> {
        let claim = self.clone().decode(&self.refresh_token_secret, token).map_err(|err| err)?;
        if claim.sub != "refresh" {
            return Err(super::Error {
                cause: String::from("invalid refresh token. token subject is not refresh"),
                message: String::from("invalid refresh token"),
                status: 400,
            });
        }
        let claim_data: &str = &claim.data;
        let data: T = serde_json::from_str(claim_data).map_err(|err| super::Error {
            cause: err.to_string(),
            message: String::from("failed to decode jwt token"),
            status: 500,
        })?;

        Ok(data)
    }
}
