use crate::version;
use ::log::{debug, warn};
use chrono::{FixedOffset, Local, TimeZone};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use fs_extra::dir::{copy as dir_copy, remove as dir_remove, CopyOptions};
use qmetaobject::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use tar::Archive;

#[derive(QObject, Default)]
pub struct Utility {
    base: qt_base_class!(trait QObject),

    local_time_now_qml: qt_method!(fn(&mut self, format: QString) -> QString),
    get_time_from_utc_seconds_qml: qt_method!(fn(&self, sec: i64) -> QString),
    utc_seconds_to_local_string_qml: qt_method!(fn(&self, sec: i64, format: QString) -> QString),
    copy_to_clipboard_qml: qt_method!(fn(&self, text: QString) -> bool),

    prepare_pack_qml: qt_method!(
        fn(
            &self,
            src_dir: QString,
            dst_dir: QString,
            exclude_file: QVariantList,
            exclude_dir: QVariantList,
        ) -> bool
    ),

    pack_qml: qt_method!(
        fn(
            &self,
            filename: QString,
            dir_pre: QString,
            config_dir: QString,
            data_dir: QString,
        ) -> bool
    ),

    unpack_qml: qt_method!(fn(&self, filepath: QString) -> bool),
    move_file_qml: qt_method!(fn(&self, src: QString, dst: QString) -> bool),
    move_files_qml: qt_method!(fn(&self, src_dir: QString, dst_dir: QString) -> bool),
    remove_dir_qml: qt_method!(fn(&self, dir: QString) -> bool),

    copy_dir_qml: qt_method!(fn(&self, src_dir: QString, dst_dir: QString) -> bool),
    exit_qml: qt_method!(fn(&self, code: i32)),
    process_cmd_qml: qt_method!(fn(&self, cmd: QString, args: QString) -> bool),
    app_version_qml: qt_method!(fn(&self) -> QString),
}

impl Utility {
    pub fn init_from_engine(engine: &mut QmlEngine, utility: QObjectPinned<Utility>) {
        engine.set_object_property("utility".into(), utility);
    }

    pub fn local_time_now(format: &str) -> String {
        return Local::now().format(format).to_string();
    }

    pub fn utc_seconds_to_local_string(sec: i64, format: &str) -> String {
        let time = FixedOffset::east_opt(8 * 3600)
            .unwrap()
            .timestamp_opt(sec, 0)
            .unwrap();
        return format!("{}", time.format(format));
    }

    pub fn local_time_now_qml(&mut self, format: QString) -> QString {
        return Local::now()
            .format(format.to_string().as_str())
            .to_string()
            .into();
    }

    pub fn get_time_from_utc_seconds_qml(&self, sec: i64) -> QString {
        let time = FixedOffset::east_opt(8 * 3600)
            .unwrap()
            .timestamp_opt(sec, 0)
            .unwrap();
        return format!("{}", time.format("%Y-%m-%d %H:%M")).into();
    }

    // "%y-%m-%d %H:%M"
    pub fn utc_seconds_to_local_string_qml(&self, sec: i64, format: QString) -> QString {
        let time = FixedOffset::east_opt(8 * 3600)
            .unwrap()
            .timestamp_opt(sec, 0)
            .unwrap();
        return format!("{}", time.format(format.to_string().as_ref())).into();
    }

    pub fn copy_to_clipboard_qml(&self, text: QString) -> bool {
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

    pub fn prepare_pack_qml(
        &self,
        src_dir: QString,
        dst_dir: QString,
        exclude_file: QVariantList,
        exclude_dir: QVariantList,
    ) -> bool {
        if !self.copy_dir_qml(src_dir.to_string().into(), dst_dir.to_string().into()) {
            warn!(
                "can not copy dir: {} => {}",
                src_dir.to_string(),
                dst_dir.to_string()
            );
            return false;
        }

        for item in exclude_file.into_iter() {
            let item = item.to_qbytearray().to_string();
            let file = format!("{}/{}", dst_dir, item);
            let _ = fs::remove_file(file);
        }

        for item in exclude_dir.into_iter() {
            let item = item.to_qbytearray().to_string();
            let dir = format!("{}/{}", dst_dir, item);
            debug!("{}", &dir);
            let _ = fs::remove_dir_all(dir);
        }
        return true;
    }
    pub fn pack_qml(
        &self,
        filename: QString,
        dir_pre: QString,
        config_dir: QString,
        data_dir: QString,
    ) -> bool {
        match File::create(&filename.to_string()) {
            Ok(tar) => {
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
            Err(e) => warn!("{:?}", e),
        }
        return false;
    }

    pub fn unpack_qml(&self, filepath: QString) -> bool {
        let filepath = filepath.to_string();
        match File::open(&filepath) {
            Ok(tar_gz) => {
                let tar = GzDecoder::new(tar_gz);
                let mut archive = Archive::new(tar);
                if archive.unpack(".").is_ok() {
                    return true;
                } else {
                    warn!("upack {} error!", filepath);
                }
            }
            Err(e) => warn!("open {} failed, error: {:?}", filepath, e),
        }
        return false;
    }

    pub fn move_file_qml(&self, src: QString, dst: QString) -> bool {
        return fs::rename(src.to_string(), dst.to_string()).is_ok();
    }

    pub fn remove_dir_qml(&self, dir: QString) -> bool {
        dir_remove(&dir.to_string()).is_ok()
    }

    pub fn copy_dir_qml(&self, src_dir: QString, dst_dir: QString) -> bool {
        let s_dir = src_dir.to_string();
        let s_dir = Path::new(&s_dir);
        let d_dir = dst_dir.to_string();
        let d_dir = Path::new(&d_dir);

        if !d_dir.exists() && fs::create_dir_all(&d_dir).is_err() {
            return false;
        }
        let mut op = CopyOptions::new();
        op.overwrite = true;
        match dir_copy(s_dir, d_dir, &op) {
            Err(e) => {
                warn!(
                    "copy dir {} => {} failed. error: {:?}",
                    src_dir.to_string(),
                    dst_dir.to_string(),
                    e
                );
                return false;
            }
            _ => {
                debug!(
                    "copy dir {} => {} successfully",
                    src_dir.to_string(),
                    dst_dir.to_string()
                );
                return true;
            }
        }
    }

    pub fn move_files_qml(&self, src_dir: QString, dst_dir: QString) -> bool {
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
                if !self.move_files_qml(src_dir, dst_dir) {
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

    pub fn exit_qml(&self, code: i32) {
        std::process::exit(code);
    }

    pub fn process_cmd_qml(&self, cmd: QString, args: QString) -> bool {
        let args = args.to_string();
        let args = args.split(',');
        return Command::new(cmd.to_string()).args(args).spawn().is_ok();
    }

    pub fn app_version_qml(&self) -> QString {
        return version::VERSION.to_string().into();
    }
}
