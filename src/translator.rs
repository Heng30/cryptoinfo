use qmetaobject::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Translator {
    base: qt_base_class!(trait QObject),
    path: String,
    lang_map: HashMap<String, String>,

    use_chinese: qt_property!(bool; NOTIFY use_chinese_changed),
    use_chinese_changed: qt_signal!(),

    tr: qt_method!(fn(&mut self, text: QString) -> QString),
}

impl Translator {
    pub fn init_from_engine(engine: &mut QmlEngine, translator: QObjectPinned<Translator>) {
        engine.set_object_property("translator".into(), translator);
    }

    pub fn load(&mut self) {
        if let Ok(file) = File::open(&self.path) {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Err(_) = line {
                    continue;
                }

                let item = line
                    .unwrap()
                    .split(',')
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                if item.len() != 2 {
                    continue;
                }
                self.lang_map.insert(item[0].clone(), item[1].clone());
            }
        } else {
            warn!("can not load translation file: {}", &self.path);
        }
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    pub fn set_use_chinese(&mut self, use_chinese: bool) {
        self.use_chinese = use_chinese;
    }

    fn tr(&self, text: QString) -> QString {
        if self.use_chinese {
            return text;
        }
        if let Some(value) = self.lang_map.get(&text.to_string()) {
            return value.to_string().into();
        }
        return text;
    }
}
