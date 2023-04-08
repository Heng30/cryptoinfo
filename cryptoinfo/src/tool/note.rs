use crate::qobjmgr::{qobj, NodeType as QNodeType};
use ::log::{debug, warn};
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;
use std::fs;
use std::path::Path;
use NoteItem as Item;

#[derive(QGadget, Clone, Default)]
pub struct NoteItem {
    name: qt_property!(QString),
}

modeldata_struct!(Model, Item, members: {
        dir: String,
    }, members_qt: {
        text: [QString; text_changed],
    }, signals_qt: {
    }, methods_qt: {
        load_qml: fn(&mut self, index: i32),
        save_qml: fn(&mut self, index: i32, text: QString),
        add_item_qml: fn(&mut self, name: QString),
        set_item_qml: fn(&mut self, index: i32, name: QString),
        remove_item_qml: fn(&mut self, index: i32),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.dir = app_dirs
            .data_dir
            .join("notes")
            .to_str()
            .unwrap()
            .to_string();
        self.load_names();
    }

    fn load_names(&mut self) {
        let dir = Path::new(&self.dir);
        if !dir.exists() {
            return;
        }

        match fs::read_dir(&dir) {
            Err(e) => debug!("{:?}", e),
            Ok(dirs) => {
                for entry in dirs {
                    match entry {
                        Err(e) => debug!("{:?}", e),
                        Ok(entry) => match entry.file_name().into_string() {
                            Err(e) => debug!("{:?}", e),
                            Ok(name) => {
                                let v = name.split('.').collect::<Vec<_>>();
                                if v.len() != 2 {
                                    continue;
                                }

                                self.append(Item {
                                    name: v[0].to_string().into(),
                                });
                            }
                        },
                    }
                }

                self.items_mut()
                    .sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));
            }
        }
    }

    fn load_qml(&mut self, index: i32) {
        if index < 0 || index as usize >= self.items_len() {
            return;
        }

        let index = index as usize;
        let path = self.dir.clone() + "/" + &self.items()[index].name.to_string() + ".md";

        match std::fs::read_to_string(&path) {
            Ok(text) => {
                self.text = text.into();
            }
            Err(e) => {
                self.text = "".to_string().into();
                debug!("{:?}", e);
            }
        }
        self.text_changed();
    }

    fn save_qml(&mut self, index: i32, text: QString) {
        if index < 0 || index as usize >= self.items_len() {
            return;
        }

        let index = index as usize;
        let path = self.dir.clone() + "/" + &self.items()[index].name.to_string() + ".md";

        self.text = text;
        self.text_changed();
        if fs::write(&path, self.text.to_string()).is_err() {
            warn!("save {:?} failed", &path);
        }
    }

    fn add_item_qml(&mut self, name: QString) {
        if name.is_empty() {
            return;
        }

        let mut name = name;
        while self.items_len() > 0 {
            let mut cindex = 0;
            for item in self.items() {
                if name == item.name {
                    name = format!("{}-copy", name).into();
                    break;
                }
                cindex += 1;
            }

            if cindex >= self.items_len() {
                break;
            }
        }

        let path = self.dir.clone() + "/" + &name.to_string() + ".md";
        match fs::write(&path, "") {
            Err(e) => debug!("{:?}", e),
            _ => self.append(Item { name }),
        }
    }

    fn set_item_qml(&mut self, index: i32, name: QString) {
        if index < 0 || index as usize >= self.items_len() {
            return;
        }
        let index = index as usize;
        let opath = self.dir.clone() + "/" + &self.items()[index].name.to_string() + ".md";
        let npath = self.dir.clone() + "/" + &name.to_string() + ".md";

        if opath == npath {
            return;
        }

        for item in self.items() {
            if item.name == name {
                return;
            }
        }

        match fs::rename(opath, npath) {
            Err(e) => debug!("{:?}", e),
            _ => self.set(index, Item { name }),
        }
    }

    fn remove_item_qml(&mut self, index: i32) {
        if index < 0 || index as usize >= self.items_len() {
            return;
        }
        let index = index as usize;
        let path = self.dir.clone() + "/" + &self.items()[index].name.to_string() + ".md";

        match fs::remove_file(&path) {
            Err(e) => debug!("{:?}", e),
            _ => {
                self.remove_rows(index, 1);
            }
        }
    }
}
