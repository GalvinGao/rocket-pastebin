use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Paste {
    pub id: i32,
    pub slug: String,
    pub delete_token: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
