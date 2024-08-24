use std::env;

pub struct Config {
    pub db_url: String,
    pub auth_token_secret: String,
    pub refresh_token_secret: String,
    pub aes_iv: String,
    pub aes_key: String,
}

impl Config {
    pub fn new() -> Self {
        return Config {
            db_url: env::var("DATABASE_URL").unwrap(),
            auth_token_secret: env::var("AUTH_TOKEN_JWT_SECRET").unwrap(),
            refresh_token_secret: env::var("REFRESH_AUTH_TOKEN_JWT_SECRET").unwrap(),
            aes_iv: env::var("AES_IV").unwrap(),
            aes_key: env::var("AES_KEY").unwrap(),
        };
    }
}
