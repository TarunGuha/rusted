use actix_web::{HttpResponse, Scope, web};

use crate::services::auth::interactions::authorize::Authorize;
use crate::services::auth::schemas::authorize::AuthorizeRequest;

pub async fn authorize_api(payload: web::Query<AuthorizeRequest>) -> HttpResponse {
    let response = Authorize::new().execute(&payload);
    HttpResponse::Ok().json(response)
}

pub fn auth_router() -> Scope {
    web::scope("/auth").route("/authorize", web::get().to(authorize_api))
}
