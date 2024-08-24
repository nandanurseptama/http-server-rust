use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{self, HttpResponse};
use serde::Serialize;

use crate::responses::RestResponse;

pub mod auth_controller;

struct Response;

impl Response {
    fn send_from_rest<T: Serialize>(
        res: RestResponse<T, String>,
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
