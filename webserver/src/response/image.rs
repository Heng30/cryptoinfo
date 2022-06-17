use crate::cache::staticfile;
use rocket::http::{ContentType, Status};
use rocket::response::{Body, Responder, Response, Result};
use rocket::Request;
use std::io::Cursor;

pub struct Png {
    path: String,
}

impl Png {
    pub fn new(path: &str) -> Png {
        Png {
            path: path.to_string(),
        }
    }
}

impl<'a> Responder<'a> for Png {
    fn respond_to(self, _: &Request) -> Result<'a> {
        if !self.path.contains(".png") {
            return Err(Status::NotFound);
        }

        let data = match staticfile::bin_cache(&self.path) {
            Some(data) => data,
            None => return Err(Status::NotFound),
        };
        let len = data.len() as u64;

        Response::build()
            .header(ContentType::PNG)
            .raw_body(Body::Sized(Cursor::new(data), len))
            .ok()
    }
}
