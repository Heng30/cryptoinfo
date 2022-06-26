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

lazy_static! {
    static ref BINCACHEMAP: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

fn load_cache(filepath: &str) -> Result<String, io::Error> {
    let path = match filepath {
        "/apiv1/coin/price" => "price.json".to_string(),
        "/apiv1/coin/private" => "private.json".to_string(),
        "/apiv1/coin/btc-next-halving-day-left" => "btc-next-halving-day-left.json".to_string(),
        "/apiv1/fear-greed" => "fear-greed.json".to_string(),
        "/apiv1/market" => "market.json".to_string(),
        "/apiv1/defi/protocols" => "defi-protocols.json".to_string(),
        _ => "webserver".to_string() + filepath,
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

fn load_bin_cache(filepath: &str) -> io::Result<Vec<u8>> {
    let path = "webserver".to_string() + filepath;
    let path = APPDIR
        .lock()
        .unwrap()
        .data_dir
        .join(path)
        .to_str()
        .unwrap()
        .to_string();

    return fs::read(&path);
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

    return Some(CACHEMAP.lock().unwrap().get(filepath).unwrap().clone());
}

pub fn bin_cache(filepath: &str) -> Option<Vec<u8>> {
    if !BINCACHEMAP
        .lock()
        .unwrap()
        .contains_key(&filepath.to_string())
    {
        if let Ok(data) = load_bin_cache(filepath) {
            BINCACHEMAP
                .lock()
                .unwrap()
                .insert(filepath.to_string(), data.clone());
            return Some(data);
        }
        return None;
    }

    return Some(BINCACHEMAP.lock().unwrap().get(filepath).unwrap().clone());
}

pub fn timer_cache(filepath: &str) -> Option<String> {
    let has_key = {
        TIMERCACHEMAP
            .lock()
            .unwrap()
            .contains_key(&filepath.to_string())
    };

    if !has_key || is_timeout(filepath) {
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
            .get(filepath)
            .unwrap()
            .jtext
            .clone(),
    );
}

fn is_timeout(filepath: &str) -> bool {
    let timeout = match filepath {
        "/apiv1/coin/price" => 10,
        "/apiv1/fear_greed" | "/apiv1/market" | "/apiv1/coin/private" => 10,
        _ => 1,
    };

    match TIMERCACHEMAP.lock().unwrap().get(&filepath.to_string()) {
        Some(v) => {
            return (Local::now().timestamp() - v.ltime.timestamp()) > timeout;
        }
        None => return true,
    }
}
