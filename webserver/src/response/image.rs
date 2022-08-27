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
        let mut ctype = ContentType::PNG;
        if self.path.contains(".ico") {
            ctype = ContentType::Icon;
        }

        let data = match staticfile::load_binary(&self.path) {
            Ok(data) => data,
            Err(_) => return Err(Status::NotFound),
        };
        let len = data.len() as u64;

        Response::build()
            .header(ctype)
            .raw_body(Body::Sized(Cursor::new(data), len))
            .ok()
    }
}
