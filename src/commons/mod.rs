pub mod config;
pub mod jwt;
pub mod aes;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub cause: String,
    pub status : u16,
}