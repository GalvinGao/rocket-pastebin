use rocket::{
    response::Responder,
    serde::{Deserialize, Serialize},
};

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

#[derive(Debug, Clone, Deserialize)]
pub struct UploadReq<'a> {
    pub content: &'a str,
}
