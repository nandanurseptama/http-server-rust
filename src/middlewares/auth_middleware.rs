use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    Error, HttpMessage,
};

use crate::{responses, services::AuthService};

pub async fn auth_middleware(
    auth_service: actix_web::web::Data<AuthService>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let authorization_header_opt = req.headers().get(AUTHORIZATION);
    if authorization_header_opt.is_none() {
        let body = responses::RestBody::<String, String> {
            status: 401,
            data: Option::None,
            error: Option::Some(String::from("authorization header required")),
            message: String::from("authorization header required"),
        };
        return Err(Error::from(responses::Response::from_rest(body)));
    }
    let authorization_header = authorization_header_opt.unwrap();
    let auth_token = authorization_header.to_str().unwrap();
    let srv = auth_service.as_ref().clone();
    let auth_result = srv.auth(String::from(auth_token)).await.map_err(|err| {
        let body = responses::RestBody::<String, String> {
            status: err.status,
            data: Option::None,
            error: Option::Some(err.cause),
            message: String::from(err.message),
        };
        return Error::from(responses::Response::from_rest(body));
    })?;
    req.extensions_mut().insert(auth_result);

    next.call(req).await
}
