use crate::services::auth::enums::ResourceMethod;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AuthorizeRequest {
    #[validate(length(min = 1, max = 100))]
    pub resource_name: String,
    pub resource_method: ResourceMethod,
    #[validate(length(min = 1, max = 100))]
    pub resource_service: String,
    pub auth_token: Option<String>,
}

#[derive(Serialize)]
pub struct AuthorizeResponse {
    pub is_authorized: bool,
    pub status_code: u16,
    pub status_message: String,
}
