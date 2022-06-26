use crate::cache::staticfile;
use rocket::http::ContentType;
use rocket::response::{Body, Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

pub struct Protocol {
    path: String,
}

impl Protocol {
    pub fn new(path: &str) -> Protocol {
        Protocol {
            path: path.to_string(),
        }
    }
}

impl<'a> Responder<'a> for Protocol {
    fn respond_to(self, _: &Request) -> Result<'a> {
        let text = match staticfile::timer_cache(&self.path) {
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


