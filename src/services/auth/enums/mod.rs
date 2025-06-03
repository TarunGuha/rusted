use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}
