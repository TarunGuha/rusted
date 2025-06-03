use crate::services::auth::enums::ResourceMethod;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthorizeRequest {
    pub resource_name: String,
    pub resource_method: ResourceMethod,
    pub resource_service: String,
    pub auth_token: Option<String>,
}

#[derive(Serialize)]
pub struct AuthorizeResponse {
    pub is_authorized: bool,
    pub status_code: u16,
    pub status_message: String,
}
