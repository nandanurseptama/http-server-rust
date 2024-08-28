use actix_web::{
    body::BoxBody, get, middleware::from_fn, HttpMessage, HttpRequest, HttpResponse, ResponseError,
};

use crate::{middlewares, responses};

#[get("/me")]
async fn me(req: HttpRequest) -> HttpResponse<BoxBody> {
    let extensions = req.extensions();
    let get_user = extensions.get::<responses::auth::User>();
    if get_user.is_none() {
        return responses::Response::from_rest(responses::RestBody::<
            responses::auth::User,
            String,
        > {
            status: 401,
            data: Option::None,
            error: Option::Some(String::from("unauthorized")),
            message: String::from("unauthorized"),
        })
        .error_response();
    }
    let user = get_user.unwrap().clone();
    return responses::Response::send_from_rest(responses::RestBody {
        status: 200,
        data: Option::Some(user),
        error: Option::None,
        message: String::from("OK"),
    });
}

pub fn config(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        actix_web::web::scope("/user")
            .wrap(from_fn(middlewares::auth_middleware::auth_middleware))
            .service(me),
    );
}
