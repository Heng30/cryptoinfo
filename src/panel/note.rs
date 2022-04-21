use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(QObject, Default)]
pub struct Note {
    base: qt_base_class!(trait QObject),

    note_path: String,

    text: qt_property!(QString; NOTIFY text_changed),
    text_changed: qt_signal!(),

    save_text: qt_method!(fn(&mut self, text: QString)),
}

impl Note {
    pub fn init_from_engine(engine: &mut QmlEngine, note: QObjectPinned<Note>) {
        engine.set_object_property("private_note".into(), note);
    }

    pub fn set_note_path(&mut self, path: &str) {
        self.note_path = path.to_string();
    }

    #[allow(unused)]
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string().into();
    }

    #[allow(unused)]
    pub fn get_text(&self) -> String {
        return self.text.to_string();
    }

    pub fn load_text(&mut self) {
        if self.note_path.is_empty() {
            return;
        }

        if let Ok(text) = std::fs::read_to_string(&self.note_path) {
            self.text = text.into();
        } else {
            warn!("load {:?} failed", &self.note_path)
        }
    }

    fn save_text(&mut self, text: QString) {
        if self.note_path.is_empty() {
            return;
        }

        self.text = text;
        self.text_changed();
        if let Err(_) = std::fs::write(&self.note_path, self.text.to_string()) {
            warn!("save {:?} failed", &self.note_path);
            return;
        }
    }
}
