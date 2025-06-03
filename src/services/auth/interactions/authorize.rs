use crate::services::auth::schemas::authorize::{AuthorizeRequest, AuthorizeResponse};
use actix_web::HttpResponse;
use validator::Validate;

pub struct Authorize;

impl Authorize {
    pub fn new() -> Self {
        Authorize
    }

    pub fn set_request(&self, request: &AuthorizeRequest) -> Result<(), HttpResponse> {
        request
            .validate()
            .map_err(|err| HttpResponse::UnprocessableEntity().json(err))
    }

    pub fn execute(&self, request: &AuthorizeRequest) -> Result<AuthorizeResponse, HttpResponse> {
        self.set_request(request)?;

        let token = format!(
            "{} {:?} {} {}",
            request.resource_name,
            request.resource_method,
            request.resource_service,
            request.auth_token.as_deref().unwrap_or_default()
        );

        Ok(AuthorizeResponse {
            is_authorized: true,
            status_code: 200,
            status_message: token,
        })
    }
}
