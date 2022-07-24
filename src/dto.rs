use rocket::{response::Responder, serde::Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadResp {
    pub url: String,
    pub delete_token: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteSucceededResp {
    pub deleted_slug: String,
}

#[derive(Debug, Clone, Serialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    pub error: String,
}
