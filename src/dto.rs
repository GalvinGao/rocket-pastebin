use rocket::{
    response::Responder,
    serde::{Deserialize, Serialize},
};
use rocket_validation::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct UploadResp {
    pub slug: String,
    pub delete_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteSucceededResp {
    pub deleted_slug: String,
}

#[derive(Debug, Clone, Serialize, Responder)]
pub struct Error {
    pub error: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UploadReq<'a> {
    #[validate(length(min = 1))]
    pub content: &'a str,
}
