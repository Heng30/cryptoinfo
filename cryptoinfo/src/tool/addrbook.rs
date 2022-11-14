use crate::qobjmgr::{qobj, NodeType as QNodeType};
use image::Rgb;
use modeldata::*;
use qmetaobject::*;
use qrcode::QrCode;

#[allow(unused_imports)]
use ::log::{debug, warn};
use crypto_hash::{hex_digest, Algorithm};
use platform_dirs::AppDirs;
use serde_derive::{Deserialize, Serialize};
use AddrBookItem as Item;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawItem {
    name: String,
    addr: String,
}

#[derive(QGadget, Clone, Default)]
pub struct AddrBookItem {
    name: qt_property!(QString),
    addr: qt_property!(QString),
    image_path: qt_property!(QString),
}

modeldata_struct!(Model, Item, members: {
        dir: String,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
        save_qml: fn(&mut self),
        add_item_qml: fn(&mut self, name: QString, addr: QString),
        set_item_qml: fn(&mut self, index: usize, name: QString, addr: QString),
        refresh_item_qml: fn(&mut self, index: usize, name: QString, addr: QString),
        remove_item_qml: fn(&mut self, index: usize),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.dir = app_dirs
            .data_dir
            .join("addrbook")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

    pub fn load(&mut self) {
        let path = self.dir.clone() + "/data.json";

        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(mut raw_items) = serde_json::from_str::<Vec<RawItem>>(&text) {
                raw_items.sort_by(|a, b| a.name.cmp(&b.name));

                for item in &raw_items {
                    self.add_item(item);
                }
            }
        }
    }

    fn save_qml(&mut self) {
        let mut raw_items = vec![];
        for item in self.items() {
            raw_items.push(RawItem {
                name: item.name.to_string(),
                addr: item.addr.to_string(),
            });
        }

        let path = self.dir.clone() + "/data.json";
        if let Ok(text) = serde_json::to_string_pretty(&raw_items) {
            if let Err(_) = std::fs::write(&path, text) {
                warn!("save {:?} failed", &path);
            }
        }
    }

    fn new_item(&self, name: &str, addr: &str) -> Item {
        let name_image = hex_digest(Algorithm::MD5, name.to_string().as_bytes());
        let image_path = self.dir.clone() + "/" + &name_image + ".png";

        return Item {
            name: name.to_string().into(),
            addr: addr.to_string().into(),
            image_path: image_path.into(),
        };
    }

    fn add_item(&mut self, raw_item: &RawItem) {
        let item = self.new_item(&raw_item.name, &raw_item.addr);
        self.append(item);
    }

    fn qrcode_image(&self, path: &str, addr: &str) -> bool {
        if let Ok(code) = QrCode::new(addr) {
            let image = code.render::<Rgb<u8>>().build();
            if let Err(e) = image.save(path) {
                debug!("gen qrcode image error: {:?}", e);
            } else {
                return true;
            }
        } else {
            debug!("qrcode new error");
        }

        return false;
    }

    fn remove_qrcode_image(&self, index: usize) {
        if index >= self.items_len() {
            return;
        }
        let path = &self.items()[index].image_path.to_string();
        let _ = std::fs::remove_file(&path);
    }

    fn add_item_qml(&mut self, name: QString, addr: QString) {
        let item = self.new_item(&name.to_string(), &addr.to_string());
        self.qrcode_image(&item.image_path.to_string(), &addr.to_string());
        self.append(item);
    }

    fn set_item_qml(&mut self, index: usize, name: QString, addr: QString) {
        let item = self.new_item(&name.to_string(), &addr.to_string());
        self.remove_qrcode_image(index);
        self.qrcode_image(&item.image_path.to_string(), &addr.to_string());
        self.set(index, item);
    }

    fn remove_item_qml(&mut self, index: usize) {
        self.remove_qrcode_image(index);
        self.remove_rows(index, 1);
    }

    fn refresh_item_qml(&mut self, index: usize, name: QString, addr: QString) {
        self.set_item_qml(index, name, addr);
    }
}
