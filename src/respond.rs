use rocket::serde::json::Json;

#[derive(Responder)]
pub enum Error<T> {
    #[response(status = 500)]
    Unexpected(T),
}
