use crate::cache::staticfile;
use rocket::http::{ContentType, Status};
use rocket::response::{Body, Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

pub struct Data {
    path: String,
}

impl Data {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl<'a> Responder<'a> for Data {
    fn respond_to(self, _: &Request) -> Result<'a> {
        let text = match staticfile::load_text(&self.path) {
            Ok(text) => text,
            Err(_) => return Err(Status::NotFound),
        };
        let len = text.len() as u64;

        Response::build()
            .header(ContentType::JSON)
            .raw_body(Body::Sized(Cursor::new(text), len))
            .ok()
    }
}
