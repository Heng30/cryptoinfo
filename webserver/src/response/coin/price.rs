use crate::middleware::cache::timer_cache;
use rocket::http::ContentType;
use rocket::response::{Body, Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

pub struct Price {}

impl<'a> Responder<'a> for Price {
    fn respond_to(self, _: &Request) -> Result<'a> {
        let text = match timer_cache("/coin/price") {
            Some(text) => text,
            None => "[]".to_string(),
        };
        let len = text.len() as u64;

        Response::build()
            .header(ContentType::JSON)
            .raw_body(Body::Sized(Cursor::new(text), len))
            .ok()
    }
}
