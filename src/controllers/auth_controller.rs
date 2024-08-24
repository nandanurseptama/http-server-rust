use actix_web::{body::BoxBody, post, HttpResponse};

use crate::{requests::RegisterRequest, services::AuthService};

#[post("/register")]
async fn register(
    data: actix_web::web::Data<AuthService>,
    req: actix_web::web::Json<RegisterRequest>,
) -> HttpResponse<BoxBody> {
    let result = data.register(req.0).await;
    return super::Response::send_from_service(result);
}

pub fn config(config: &mut actix_web::web::ServiceConfig) {
    config.service(actix_web::web::scope("/auth").service(self::register));
}
