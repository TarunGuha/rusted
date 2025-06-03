use crate::services::auth::schemas::authorize::{AuthorizeRequest, AuthorizeResponse};

pub struct Authorize;

impl Authorize {
    pub fn new() -> Self {
        Authorize
    }

    pub fn execute(&self, request: &AuthorizeRequest) -> AuthorizeResponse {
        let token = format!(
            "{} {:?} {} {}",
            request.resource_name,
            request.resource_method,
            request.resource_service,
            request.auth_token.clone().unwrap_or_default()
        );

        AuthorizeResponse {
            is_authorized: true,
            status_code: 200,
            status_message: token,
        }
    }
}
