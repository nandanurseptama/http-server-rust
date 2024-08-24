use serde::Serialize;

pub mod auth;

#[derive(Debug, Serialize)]
pub struct RestResponse<Data: Serialize, Err: ToString> {
    pub status: u16,
    pub data: Option<Data>,
    pub error: Option<Err>,
    pub message: String,
}
