use crate::models::Paste;
use diesel;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket_todomvc::schema::pastes::dsl;

pub fn get_paste(id: String, conn: &PgConnection) -> Option<Paste> {
    dsl::pastes
        .filter(dsl::slug.eq(id))
        .first::<Paste>(conn)
        .ok()
}

pub fn get_pastes(conn: &PgConnection) -> Result<Vec<Paste>, diesel::result::Error> {
    dsl::pastes
        .filter(dsl::deleted_at.is_null())
        .load::<Paste>(conn)
}

pub fn insert_pastes(
    id: String,
    content: String,
    delete_token: String,
    conn: &PgConnection,
) -> Option<usize> {
    insert_into(dsl::pastes)
        .values((
            dsl::slug.eq(id),
            dsl::content.eq(content),
            dsl::delete_token.eq(delete_token),
        ))
        .execute(conn)
        .ok()
}

pub fn delete_paste(id: String, conn: &PgConnection) -> Result<(), diesel::result::Error> {
    delete(dsl::pastes)
        .filter(dsl::slug.eq(id))
        .execute(conn)
        .map(|_| ())
}
