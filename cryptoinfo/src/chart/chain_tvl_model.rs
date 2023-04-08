use super::data::{ChainTVLItem as Item, RawChainTVLItem as RawItem};
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
use ::log::debug;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type MName = Mutex<Option<String>>;
type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        dir: String,
        update_now: AtomicBool,
        name: MName,
    }, members_qt: {
        min_tvl: [u64; min_tvl_changed],
        max_tvl: [u64; max_tvl_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        is_updating_qml: fn(&mut self) -> bool,
        name_qml: fn(&self) -> QString,
        set_name_qml: fn(&mut self, name: QString),
        refresh_qml: fn(&mut self),
        use_cache_data_qml: fn(&mut self),
        likely_item_qml: fn(&mut self, second: u64) -> QVariant,
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow_mut().gen_url();
    }

    fn update_interval(&self) -> usize {
        return usize::max_value();
    }

    fn update_now(&self) -> bool {
        return self.borrow().update_now.load(AOrdering::SeqCst);
    }

    fn disable_update_now(&self) {
        self.borrow().update_now.store(false, AOrdering::SeqCst);
    }

    fn parse_body(&mut self, text: &str) {
        self.borrow_mut().save(text);
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.dir = app_dirs
            .data_dir
            .join("tmp/chain-tvl")
            .to_str()
            .unwrap()
            .to_string();
        self.async_update_model();
    }

    fn gen_path(&self) -> String {
        if let Some(name) = (*self.name.lock().unwrap()).as_ref() {
            self.dir.clone() + "/" + name + ".json"
        } else {
            "".to_string()
        }
    }

    pub fn gen_url(&self) -> String {
        if let Some(name) = (*self.name.lock().unwrap()).as_ref() {
            if name == "Chains" {
                "https://api.llama.fi/charts".to_string()
            } else {
                "https://api.llama.fi/charts/".to_string() + name
            }
        } else {
            "".to_string()
        }
    }

    fn update_model(&mut self, _text: String) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        self.clear();
        for item in tmp_items.unwrap() {
            self.append(item);
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
        self.updated();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 30, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(raw_item) => {
                let mut min_tvl = u64::max_value();
                let mut max_tvl = 0;
                let mut v = vec![];

                for (i, item) in raw_item.into_iter().enumerate() {
                    min_tvl = u64::min(item.tvl as u64, min_tvl);
                    max_tvl = u64::max(item.tvl as u64, max_tvl);

                    let mut item = Self::new_item(item);
                    item.index = i as i32;
                    v.push(item);
                }

                if max_tvl > min_tvl {
                    self.max_tvl = max_tvl;
                    self.min_tvl = min_tvl;
                    self.max_tvl_changed();
                    self.min_tvl_changed();
                }
                *self.tmp_items.lock().unwrap() = Some(v);
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    fn new_item(raw_item: RawItem) -> Item {
        return Item {
            second: raw_item.date.parse().unwrap_or(0),
            tvl: raw_item.tvl as u64,
            ..Default::default()
        };
    }

    fn load(&mut self) -> Option<String> {
        let file = self.gen_path();
        match std::fs::read_to_string(&file) {
            Ok(text) => Some(text),
            Err(e) => {
                debug!("file: {:?}, error: {:?}", &file, e);
                None
            }
        }
    }

    fn save(&self, text: &str) {
        let file = self.gen_path();
        if let Err(e) = std::fs::write(&file, text) {
            debug!("{:?}", e);
        }
    }

    fn use_cache_data_qml(&mut self) {
        match self.load() {
            None => {
                self.clear();
                self.refresh_qml();
            }
            Some(text) => {
                self.cache_items(&text);
                self.update_model(text);
            }
        }
    }

    fn is_updating_qml(&mut self) -> bool {
        self.update_now.load(AOrdering::SeqCst)
    }

    fn set_name_qml(&mut self, name: QString) {
        *self.name.lock().unwrap() = Some(name.to_string());
    }

    fn name_qml(&self) -> QString {
        if let Some(name) = (*self.name.lock().unwrap()).as_ref() {
            name.clone().into()
        } else {
            "N/A".to_string().into()
        }
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn likely_item_qml(&mut self, second: u64) -> QVariant {
        if self.items_is_empty() {
            return Item::default().to_qvariant();
        }

        let mut s = 0_usize;
        let mut e = self.items_len() as usize;
        let mut m = s / 2 + e / 2_usize;
        while s < e {
            let item = &self.items()[m];
            if item.second == second {
                return item.to_qvariant();
            } else if item.second < second {
                s = m + 1;
            } else {
                e = m - 1;
            }
            m = s / 2 + e / 2_usize;
        }

        if s >= self.items_len() {
            return self.items()[self.items_len()].to_qvariant();
        }

        return self.items()[s].to_qvariant();
    }
}
