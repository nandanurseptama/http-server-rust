use std::fmt::Display;

use actix_web::{
    body::BoxBody,
    http::{self, header::ContentType, StatusCode},
    HttpResponse, HttpResponseBuilder, Responder, ResponseError,
};
use serde::Serialize;

pub mod auth;

#[derive(Debug, Serialize)]
pub struct RestBody<Data: Serialize, Err: ToString> {
    pub status: u16,
    pub data: Option<Data>,
    pub error: Option<Err>,
    pub message: String,
}

#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, body: String) -> Self {
        Response {
            status_code: StatusCode::from_u16(status_code).unwrap(),
            body,
        }
    }
    pub fn from_rest<Data: Serialize>(res: RestBody<Data, String>) -> Self {
        let body_string = serde_json::to_string(&res);
        Self {
            status_code: http::StatusCode::from_u16(res.status).unwrap(),
            body: body_string.unwrap(),
        }
    }
    pub fn send_from_rest<T: Serialize>(
        res: RestBody<T, String>,
    ) -> actix_web::HttpResponse<BoxBody> {
        let body_string = serde_json::to_string(&res);
        let body = actix_web::body::BoxBody::new(actix_web::web::BytesMut::from(
            body_string.unwrap().as_bytes(),
        ));
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(actix_web::http::StatusCode::from_u16(res.status).unwrap())
            .body(body);
    }
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(actix_web::web::BytesMut::from(self.body.as_bytes()));
        HttpResponseBuilder::new(self.status_code)
            .content_type(ContentType::json())
            .body(body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {} \n Status Code: {}",
            self.body, self.status_code
        )
    }
}

impl ResponseError for Response {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(actix_web::web::BytesMut::from(self.body.as_bytes()));
        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::json())
            .body(body)
    }
}
