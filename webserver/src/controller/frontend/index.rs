use crate::middleware::cache::cache;
use rocket::http::{ContentType, Status};
use rocket::response::{Body, Response, Result};
use std::io::Cursor;

fn response<'a>(filepath: &str, ctype: ContentType) -> Result<'a> {
    let text = match cache(filepath) {
        Some(text) => text,
        None => return Err(Status::NotFound),
    };

    let len = text.len() as u64;
    Response::build()
        .header(ctype)
        .raw_body(Body::Sized(Cursor::new(text), len))
        .ok()
}

#[get("/")]
pub fn index<'a>() -> Result<'a> {
    response("/index.html", ContentType::HTML)
}

#[get("/index.html")]
pub fn index_html<'a>() -> Result<'a> {
    index()
}

#[get("/css/index.css")]
pub fn css<'a>() -> Result<'a> {
    response("/css/index.css", ContentType::CSS)
}


