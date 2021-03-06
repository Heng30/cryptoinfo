use crate::qobjmgr::{qobj, NodeType as QNodeType};
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;

#[allow(unused_imports)]
use log::{debug, warn};

#[derive(QObject, Default)]
pub struct Note {
    base: qt_base_class!(trait QObject),
    path: String,

    text: qt_property!(QString; NOTIFY text_changed),
    text_changed: qt_signal!(),

    save_qml: qt_method!(fn(&mut self, text: QString)),
}

impl Note {
    pub fn init_from_engine(engine: &mut QmlEngine, note: QObjectPinned<Note>) {
        engine.set_object_property("private_note".into(), note);
    }

    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        let file = app_dirs.data_dir.join("note.md");
        self.path = file.to_str().unwrap().to_string();
        self.load();
    }

    fn load(&mut self) {
        if self.path.is_empty() {
            return;
        }

        if let Ok(text) = std::fs::read_to_string(&self.path) {
            self.text = text.into();
        } else {
            warn!("load {:?} failed", &self.path)
        }
    }

    fn save_qml(&mut self, text: QString) {
        if self.path.is_empty() {
            return;
        }

        self.text = text;
        self.text_changed();
        if let Err(_) = std::fs::write(&self.path, self.text.to_string()) {
            warn!("save {:?} failed", &self.path);
            return;
        }
    }
}
