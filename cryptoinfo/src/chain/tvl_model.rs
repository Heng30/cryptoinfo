use super::data::{RawTvlItem as RawItem, TvlItem as Item};
use super::sort::{SortDir, TvlSortKey as SortKey};
use crate::httpclient;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility::Utility;
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        chains_name_path: String,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
        tmp_items: ItemVec,
        update_now: AtomicBool,
    }, members_qt: {
        update_time: [QString; update_time_changed], //数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        refresh_qml: fn(&mut self),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        search_and_view_at_beginning_qml: fn(&mut self, text: QString),
    }
);

impl httpclient::DownloadProvider for QBox<Model> {
    fn url(&self) -> String {
        return self.borrow().url.clone();
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
        self.borrow_mut().cache_items(text);
    }
}

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(cstr!("ChainTvlSortKey"), 1, 0, cstr!("ChainTvlSortKey"));

        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.sort_key = SortKey::Index as u32;
        self.url = "https://api.llama.fi/chains".to_string();

        self.chains_name_path = app_dirs
            .data_dir
            .join("chain-names.json")
            .to_str()
            .unwrap()
            .to_string();

        self.async_update_model();
    }

    fn save_chains_name(&self, raw_items: &Vec<RawItem>) {
        let mut names: Vec<&str> = vec![];
        for item in raw_items {
            names.push(item.name.as_ref());
        }

        if names.is_empty() {
            return;
        }

        if let Ok(text) = serde_json::to_string_pretty(&names) {
            if let Err(_) = std::fs::write(&self.chains_name_path, text) {
                warn!("save {:?} failed", &self.chains_name_path);
            }
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
        self.sort_by_key_qml(self.sort_key);
        self.update_time_changed();
    }

    pub fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_pro(qptr, 5, cb);
    }

    pub fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<Vec<RawItem>>(text) {
            Ok(mut raw_item) => {
                raw_item.sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));
                self.save_chains_name(&raw_item);

                let mut v = vec![];
                for (i, item) in raw_item.into_iter().enumerate() {
                    if i >= 100 {
                        break;
                    }

                    let mut item = Self::new(item);
                    item.index = i as i32;
                    v.push(item);
                }
                *self.tmp_items.lock().unwrap() = Some(v);
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    fn refresh_qml(&mut self) {
        self.update_now.store(true, AOrdering::SeqCst);
    }

    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    fn sort_by_key_qml(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        if key == SortKey::Symbol {
            self.items_mut().sort_by(|a, b| {
                a.symbol
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.symbol.to_string().to_lowercase())
            });
        } else if key == SortKey::Name {
            self.items_mut().sort_by(|a, b| {
                a.name
                    .to_string()
                    .to_lowercase()
                    .cmp(&b.name.to_string().to_lowercase())
            });
        } else if key == SortKey::Index {
            self.items_mut().sort_by(|a, b| a.index.cmp(&b.index));
        } else if key == SortKey::TVL {
            self.items_mut()
                .sort_by(|a, b| a.tvl.partial_cmp(&b.tvl).unwrap_or(Ordering::Less));
        } else {
            return;
        }

        if self.sort_dir != SortDir::UP {
            self.items_mut().reverse();
        }
        self.sort_key = key as u32;
        self.items_changed(0, self.items_len() - 1);
    }

    fn new(raw_item: RawItem) -> Item {
        return Item {
            name: raw_item.name.into(),
            symbol: raw_item.symbol.unwrap_or("-".to_string()).into(),
            tvl: raw_item.tvl,
            ..Default::default()
        };
    }

    // 查找并与第一行交换
    fn search_and_view_at_beginning_qml(&mut self, text: QString) {
        if let Some(index) = self
            .items()
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
