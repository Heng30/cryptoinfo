use chrono::{FixedOffset, Local, TimeZone};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use qmetaobject::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use tar::Archive;

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Utility {
    base: qt_base_class!(trait QObject),

    local_time_now: qt_method!(fn(&mut self, format: QString) -> QString),
    get_time_from_utc_seconds: qt_method!(fn(&self, sec: i64) -> QString),
    utc_seconds_to_local_string: qt_method!(fn(&self, sec: i64, format: QString) -> QString),
    copy_to_clipboard: qt_method!(fn(&self, text: QString) -> bool),
    pack: qt_method!(
        fn(
            &self,
            filename: QString,
            dir_pre: QString,
            config_dir: QString,
            data_dir: QString,
        ) -> bool
    ),

    unpack: qt_method!(fn(&self, filepath: QString) -> bool),
    move_file: qt_method!(fn(&self, src: QString, dst: QString) -> bool),
    move_files: qt_method!(fn(&self, src_dir: QString, dst_dir: QString) -> bool),
    remove_dirs: qt_method!(fn(&self, dir: QString) -> bool),
}

impl Utility {
    pub fn init_from_engine(engine: &mut QmlEngine, utility: QObjectPinned<Utility>) {
        engine.set_object_property("utility".into(), utility);
    }

    pub fn local_time_now(&mut self, format: QString) -> QString {
        return format!(
            "{}",
            Local::now().format(format.to_string().as_str()).to_string()
        )
        .into();
    }

    pub fn get_time_from_utc_seconds(&self, sec: i64) -> QString {
        let time = FixedOffset::east(8 * 3600).timestamp(sec, 0);
        return format!("{}", time.format("%y-%m-%d %H:%M").to_string()).into();
    }

    // "%y-%m-%d %H:%M"
    pub fn utc_seconds_to_local_string(&self, sec: i64, format: QString) -> QString {
        let time = FixedOffset::east(8 * 3600).timestamp(sec, 0);
        return format!("{}", time.format(format.to_string().as_ref()).to_string()).into();
    }

    pub fn copy_to_clipboard(&self, text: QString) -> bool {
        let ctx: Result<ClipboardContext, _> = ClipboardProvider::new();
        if ctx.is_err() {
            return false;
        }
        let mut ctx = ctx.unwrap();
        if let Err(e) = ctx.set_contents(text.to_string()) {
            debug!("copy to clipboard error: {:?}", e);
            return false;
        }

        return true;
    }

    pub fn pack(
        &self,
        filename: QString,
        dir_pre: QString,
        config_dir: QString,
        data_dir: QString,
    ) -> bool {
        if let Ok(tar) = File::create(&filename.to_string()) {
            let enc = GzEncoder::new(tar, Compression::default());
            let mut tar = tar::Builder::new(enc);
            if tar
                .append_dir_all(dir_pre.to_string() + "/config", config_dir.to_string())
                .is_ok()
                && tar
                    .append_dir_all(dir_pre.to_string() + "/data", data_dir.to_string())
                    .is_ok()
            {
                return true;
            }
        }
        return false;
    }

    pub fn unpack(&self, filepath: QString) -> bool {
        let filepath = filepath.to_string();
        if let Ok(tar_gz) = File::open(&filepath) {
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            if archive.unpack(".").is_ok() {
                return true;
            } else {
                warn!("upack {} error!", filepath);
            }
        } else {
            warn!("open {} error!", filepath);
        }
        return false;
    }

    pub fn move_file(&self, src: QString, dst: QString) -> bool {
        return fs::rename(src.to_string(), dst.to_string()).is_ok();
    }

    pub fn remove_dirs(&self, dir: QString) -> bool {
        return fs::remove_dir_all(dir.to_string()).is_ok();
    }

    pub fn move_files(&self, src_dir: QString, dst_dir: QString) -> bool {
        let src_dir = src_dir.to_string();
        let src_dir = Path::new(&src_dir);
        let dst_dir = dst_dir.to_string();
        let dst_dir = Path::new(&dst_dir);

        if !dst_dir.exists() && fs::create_dir_all(&dst_dir).is_err() {
            return false;
        }

        let dirs = fs::read_dir(&src_dir);
        if dirs.is_err() {
            return false;
        }

        for entry in dirs.unwrap() {
            if entry.is_err() {
                return false;
            }

            let entry = entry.unwrap();
            let metadata = entry.metadata();
            if metadata.is_err() {
                return false;
            }

            if metadata.unwrap().is_dir() {
                let src_dir = src_dir
                    .join(entry.file_name())
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into();
                let dst_dir = dst_dir
                    .join(entry.file_name())
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into();
                if !self.move_files(src_dir, dst_dir) {
                    return false;
                }
            } else {
                let dst_path = dst_dir.join(entry.file_name());
                // debug!("{:?} -> {:?}", entry.path(), &dst_path);
                match fs::rename(entry.path(), &dst_path) {
                    Err(e) => {
                        warn!(
                            "{:?} -> {:?} failed! error: {:?} ",
                            entry.path(),
                            &dst_path,
                            e
                        );
                        return false;
                    }
                    _ => (),
                }
            }
        }

        return true;
    }
}
