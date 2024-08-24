use actix_web::{body::BoxBody, post, HttpResponse};

use crate::{requests::RegisterRequest, responses, services::AuthService};

#[post("/register")]
async fn register(
    data: actix_web::web::Data<AuthService>,
    req: actix_web::web::Json<RegisterRequest>,
) -> HttpResponse<BoxBody> {
    let result = data.register(req.0).await;

    let rest_response = match result {
        Ok(data) => responses::RestResponse {
            data: Option::Some(data),
            error: Option::<String>::None,
            message: String::from("OK"),
            status: 201,
        },
        Err(err) => responses::RestResponse {
            status: err.status,
            data: Option::None,
            error: Option::<String>::Some(err.cause),
            message: err.message,
        },
    };
    return super::Response::send_from_rest(rest_response);
}

#[post("/login")]
async fn login(
    data: actix_web::web::Data<AuthService>,
    req: actix_web::web::Json<RegisterRequest>,
) -> HttpResponse<BoxBody> {
    let result = data.login(req.0).await;

    let rest_response = match result {
        Ok(data) => responses::RestResponse {
            data: Option::Some(data),
            error: Option::<String>::None,
            message: String::from("OK"),
            status: 200,
        },
        Err(err) => responses::RestResponse {
            status: err.status,
            data: Option::None,
            error: Option::<String>::Some(err.cause),
            message: err.message,
        },
    };
    return super::Response::send_from_rest(rest_response);
}
pub fn config(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        actix_web::web::scope("/auth")
            .service(self::register)
            .service(self::login),
    );
}
