use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Debug, Clone, Queryable)]
pub struct Paste {
    pub id: i32,
    pub slug: String,
    pub delete_token: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
