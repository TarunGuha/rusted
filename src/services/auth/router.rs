use actix_web::{HttpResponse, Scope, web};

use crate::services::auth::interactions::authorize::Authorize;
use crate::services::auth::schemas::authorize::AuthorizeRequest;

pub async fn authorize_handler(request: web::Query<AuthorizeRequest>) -> HttpResponse {
    match Authorize::new().execute(&request) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err_response) => err_response,
    }
}

pub fn auth_router() -> Scope {
    web::scope("/auth").route("/authorize", web::get().to(authorize_handler))
}
