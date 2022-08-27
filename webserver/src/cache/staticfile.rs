use crate::APPDIR;
use std::fs;
use std::io;

pub fn load_text(filepath: &str) -> Result<String, io::Error> {
    let path = match filepath {
        "/apiv1/private.json" => "private.json".to_string(),
        "/apiv1/price.json" => "tmp/price.json".to_string(),
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

    fs::read_to_string(&path)
}

pub fn load_binary(filepath: &str) -> io::Result<Vec<u8>> {
    let path = "webserver".to_string() + filepath;
    let path = APPDIR
        .lock()
        .unwrap()
        .data_dir
        .join(path)
        .to_str()
        .unwrap()
        .to_string();

    fs::read(&path)
}
