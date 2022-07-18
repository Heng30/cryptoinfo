#![feature(proc_macro_hygiene, decl_macro, never_type)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use platform_dirs::AppDirs;

use std::sync::Mutex;

pub mod cache {
    pub mod staticfile;
}

pub mod middleware {
    pub mod counter;
    pub mod cors;
}

pub mod controller {
    pub mod frontend;
    pub mod backend;
}

pub mod response {
    pub mod apiv1;
    pub mod image;
}

lazy_static! {
    static ref APPDIR: Mutex<AppDirs> = Mutex::new(AppDirs::new(Some("cryptoinfo"), true).unwrap());
}

