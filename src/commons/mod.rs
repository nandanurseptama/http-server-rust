use serde::Serialize;

mod exception;

#[derive(Debug, Serialize)]
pub struct ServiceResult<T> {
    pub data: T,
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Serialize)]
pub struct ServiceError {
    pub cause: String,
    pub message: String,
    pub status: u16,
}
