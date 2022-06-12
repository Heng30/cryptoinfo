use crate::APPDIR;
use chrono::{DateTime, Local};
use lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::sync::Mutex;

#[allow(unused_imports)]
use log::{debug, warn};

#[derive(Debug, Clone)]
pub struct TimerCache {
    pub ltime: DateTime<Local>,
    pub jtext: String,
}

lazy_static! {
    static ref CACHEMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

lazy_static! {
    static ref TIMERCACHEMAP: Mutex<HashMap<String, TimerCache>> = Mutex::new(HashMap::new());
}

fn load_cache(filepath: &str) -> Result<String, io::Error> {
    let path = match filepath {
        "/index.html" | "/css/index.css" => "webserver".to_string() + filepath,
        "/coin/price" => "price.json".to_string(),
        _ => {
            debug!("unsport filepath: {:?}", filepath);
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "unsupport filepath",
            ));
        }
    };

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

pub fn timer_cache(filepath: &str) -> Option<String> {
    let has_key = {
        TIMERCACHEMAP
            .lock()
            .unwrap()
            .contains_key(&filepath.to_string())
    };

    if !has_key || coin_price_is_timeout(filepath) {
        if let Ok(text) = load_cache(filepath) {
            TIMERCACHEMAP.lock().unwrap().insert(
                filepath.to_string(),
                TimerCache {
                    ltime: Local::now(),
                    jtext: text.clone(),
                },
            );
            return Some(text);
        }
        return None;
    }

    return Some(
        TIMERCACHEMAP
            .lock()
            .unwrap()
            .get(&filepath.to_string())
            .unwrap()
            .jtext
            .clone(),
    );
}

fn coin_price_is_timeout(filepath: &str) -> bool {
    match TIMERCACHEMAP.lock().unwrap().get(&filepath.to_string()) {
        Some(v) => {
            return (Local::now().timestamp() - v.ltime.timestamp()) > 10;
        }
        None => return true,
    }
}
