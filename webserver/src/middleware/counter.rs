use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused_imports)]
use log::{debug, warn};

pub struct Counter {
    req: AtomicUsize,
    error: AtomicUsize,
    ok: AtomicUsize,
    get: AtomicUsize,
    post: AtomicUsize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResCounter {
    req: u64,
    error: u64,
    ok: u64,
    get: u64,
    post: u64,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            req: AtomicUsize::new(0),
            error: AtomicUsize::new(0),
            ok: AtomicUsize::new(0),
            get: AtomicUsize::new(0),
            post: AtomicUsize::new(0),
        }
    }
}

impl Fairing for Counter {
    fn info(&self) -> Info {
        Info {
            name: "Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        self.req.fetch_add(1, Ordering::Relaxed);

        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status() != Status::Ok {
            self.error.fetch_add(1, Ordering::Relaxed);
            return;
        } else {
            self.ok.fetch_add(1, Ordering::Relaxed);
        }

        if request.method() == Method::Get && request.uri().path() == "/counts" {
            if let Ok(body) = serde_json::to_string_pretty(&ResCounter {
                req: self.req.load(Ordering::Relaxed) as u64,
                error: self.error.load(Ordering::Relaxed) as u64,
                ok: self.ok.load(Ordering::Relaxed) as u64,
                get: self.get.load(Ordering::Relaxed) as u64,
                post: self.post.load(Ordering::Relaxed) as u64,
            }) {
                response.set_status(Status::Ok);
                response.set_header(ContentType::JSON);
                response.set_sized_body(Cursor::new(body));
            } else {
                response.set_status(Status::InternalServerError);
            }
        }
    }
}
