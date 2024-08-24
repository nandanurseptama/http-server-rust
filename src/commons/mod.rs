pub mod config;
pub mod jwt;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub cause: String,
    pub status : u16,
}