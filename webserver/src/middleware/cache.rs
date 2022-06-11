use crate::APPDIR;
use lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::sync::Mutex;

#[allow(unused_imports)]
use log::{debug, warn};

lazy_static! {
    static ref CACHEMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn load_cache(filepath: &str) -> Result<String, io::Error> {
    match filepath {
        "/index.html" | "/css/index.css" => {
            let path = "webserver".to_string() + filepath;
            let path = APPDIR
                .lock()
                .unwrap()
                .data_dir
                .join(path)
                .to_str()
                .unwrap()
                .to_string();

            match fs::read_to_string(&path) {
                Ok(text) => return Ok(text),
                Err(e) => {
                    warn!("read index.html error: {:?}", e);
                    return Err(e);
                }
            }
        }
        _ => {
            debug!("unsport filepath: {:?}", filepath);
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupport filepath",
            ));
        }
    };
}

pub fn cache(filepath: &str) -> Option<String> {
    if !CACHEMAP.lock().unwrap().contains_key(&filepath.to_string()) {
        if let Ok(text) = load_cache(filepath) {
            CACHEMAP
                .lock()
                .unwrap()
                .insert(filepath.to_string(), text.clone());
            return Some(text);
        }
        return None;
    }

    return Some(
        CACHEMAP
            .lock()
            .unwrap()
            .get(&filepath.to_string())
            .unwrap()
            .clone(),
    );
}
