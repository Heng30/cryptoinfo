use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused_imports)]
use log::{debug, warn};

pub struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResCounter {
    get: u64,
    post: u64,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            get: AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        }
    }
}

impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status() != Status::Ok {
            return;
        }

        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);

            if let Ok(body) = serde_json::to_string_pretty(&ResCounter {
                get: get_count as u64,
                post: post_count as u64,
            }) {
                response.set_status(Status::Ok);
                response.set_header(ContentType::JSON);
                response.set_sized_body(Cursor::new(body));
            }
        }
    }
}
