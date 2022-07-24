use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use rocket_todomvc::{models::Paste, schema::pastes::dsl};

pub fn get_pastes(id: String, conn: &PgConnection) -> Option<Paste> {
    dsl::pastes
        .filter(dsl::slug.eq(id))
        .first::<Paste>(conn)
        .ok()
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

pub fn delete_paste(id: String, conn: &PgConnection) -> Result<(), Error> {
    delete(dsl::pastes)
        .filter(dsl::slug.eq(id))
        .execute(conn)
        .map(|_| ())
}
