#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

mod dao;
mod dto;
mod models;
mod paste_id;

use diesel::prelude::*;
use paste_id::PasteId;
use rocket::{
    data::{ByteUnit, ToByteUnit},
    fs,
    http::uri::Absolute,
    response::status::{Accepted, BadRequest},
    serde::json::Json,
    Data,
};
use rocket_sync_db_pools::database;

#[cfg(test)]
mod tests;

const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[database("pastebin_db")]
pub struct DBPool(PgConnection);

#[get("/")]
async fn index() -> Option<fs::NamedFile> {
    fs::NamedFile::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "web",
        "/",
        "dist",
        "/",
        "index.html"
    ))
    .await
    .ok()
}

#[post("/", data = "<paste>")]
async fn upload<'r>(paste: Json<dto::UploadReq<'_>>, db: DBPool) -> Json<dto::UploadResp> {
    let id = PasteId::new(8);
    let paste_content = paste.content.to_string();

    let str_id: String = id.0.into();
    let str_id2 = str_id.clone();
    let slug = str_id.to_string();

    let delete_token: String = PasteId::new(32).0.into();
    let delete_token2 = delete_token.clone();

    db.run(|conn| dao::insert_pastes(str_id2, paste_content, delete_token, conn))
        .await
        .unwrap();

    Json(dto::UploadResp {
        slug,
        delete_token: delete_token2,
    })
}

#[get("/<id>")]
async fn retrieve<'r>(id: String, db: DBPool) -> Option<String> {
    db.run(|conn| dao::get_pastes(id, conn))
        .await
        .map(|v| v.content)
}

#[delete("/<id>?<token>")]
async fn delete_entries(
    id: String,
    token: String,
    db: DBPool,
) -> Option<Result<Accepted<Json<dto::DeleteSucceededResp>>, BadRequest<Json<dto::Error>>>> {
    let copied_id = id.clone();
    let copied_id2 = id.clone();
    let paste = db.run(|conn| dao::get_pastes(id, conn)).await;

    if let Some(p) = paste {
        if p.delete_token == token {
            // token succeeded
            db.run(|conn| dao::delete_paste(copied_id, conn))
                .await
                .unwrap();

            return Some(Ok(Accepted(
                Json(dto::DeleteSucceededResp {
                    deleted_slug: copied_id2,
                })
                .into(),
            )));
        } else {
            // token invalid
            return Some(Err(BadRequest(Some(Json(dto::Error {
                error: "token provided is invalid for this paste".into(),
            })))));
        }
    }

    None
}

#[launch]
fn rocket() -> _ {
    let cors = rocket_cors::CorsOptions::default()
        .to_cors()
        .expect("rocket_cors should initialize properly");

    rocket::build()
        .mount("/", routes![index, retrieve, upload, delete_entries])
        .attach(DBPool::fairing())
        .attach(cors)
}
