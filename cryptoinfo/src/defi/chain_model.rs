use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;

#[allow(unused_imports)]
use ::log::{debug, warn};

use super::data::{ChainItem as Item, RawChainItem as RawItem};
use super::sort::{ChainSortKey as SortKey, SortDir};
use crate::config::Config;
use crate::utility;

modeldata_struct!(Model, Item, {
        path: String,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, {
        text: [QString; text_changed],
        item_max_count: [u32; item_max_count_changed],
        update_interval: [u32; update_interval_changed], // 更新时间间隔
        update_now: [bool; update_now_changed], // 马上更新
        update_time: [QString; update_time_changed], //数据更新时间
    }, {
        update_all: fn(&mut self),
        sort_by_key: fn(&mut self, key: u32),
        toggle_sort_dir: fn(&mut self),
        search_and_view_at_beginning: fn(&mut self, text: QString),
    }
);

impl Model {
    pub fn init(&mut self, config: &Config, app_dirs: &AppDirs) {
        qml_register_enum::<SortKey>(cstr!("DefiChainSortKey"), 1, 0, cstr!("DefiChainSortKey"));
        self.sort_key = SortKey::Index as u32;
        self.update_interval = config.defi_refresh_interval;
        self.update_now = false;
        self.item_max_count = config.defi_item_count;
        self.url = "https://api.llama.fi/chains".to_string();

        self.path = app_dirs
            .data_dir
            .join("defi-chains.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();
    }

    // 加载本地缓存数据
    fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if text.is_empty() {
                return;
            }

            self.reset(&text);
            self.sort_by_key(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.path, text) {
            warn!("write to {} error", &self.path);
        }
    }

    // 更新model
    fn update_all(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.to_string();
        self.reset(&text);
        self.save(&text);
        self.sort_by_key(self.sort_key);
        self.update_time = utility::Utility::default().local_time_now(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text.into();
        self.text_changed();
    }

    // 设置反向搜索
    fn toggle_sort_dir(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    // 跟据key进行搜索
    fn sort_by_key(&mut self, key: u32) {
        if self.items_is_empty() {
            return;
        }

        let key: SortKey = key.into();
        match self.sort_dir {
            SortDir::UP => {
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
            }
            _ => {
                if key == SortKey::Symbol {
                    self.items_mut().sort_by(|a, b| {
                        b.symbol
                            .to_string()
                            .to_lowercase()
                            .cmp(&a.symbol.to_string().to_lowercase())
                    });
                } else if key == SortKey::Name {
                    self.items_mut().sort_by(|a, b| {
                        b.name
                            .to_string()
                            .to_lowercase()
                            .cmp(&a.name.to_string().to_lowercase())
                    });
                } else if key == SortKey::Index {
                    self.items_mut().sort_by(|a, b| b.index.cmp(&a.index));
                } else if key == SortKey::TVL {
                    self.items_mut()
                        .sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));
                } else {
                    return;
                }
            }
        }

        self.sort_key = key as u32;
        self.data_changed(0, self.items_len() - 1);
    }

    // 生成一个新条目
    fn new(raw_item: &RawItem) -> Item {
        return Item {
            name: raw_item.name.clone().into(),
            symbol: raw_item.symbol.clone().unwrap_or("-".to_string()).into(),
            tvl: raw_item.tvl,
            ..Default::default()
        };
    }

    // 添加条目
    fn add_item(&mut self, index: usize, raw_item: &RawItem) {
        let mut item = Self::new(&raw_item);
        item.index = index as i32;
        self.append(item);
    }

    // 修改条目
    fn set_item(&mut self, index: usize, raw_item: &RawItem) {
        let mut item = Self::new(&raw_item);
        item.index = index as i32;
        self.set(index, item);
    }

    // 条目不知列表中，则添加，在列表中则修改
    fn reset(&mut self, text: &str) {
        let mut raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);

        raw_item.sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));

        for (i, item) in raw_item.iter().enumerate() {
            if i >= self.item_max_count as usize {
                break;
            }

            if self.items_len() <= i {
                self.add_item(i, &item);
            } else {
                self.set_item(i, &item);
            }
        }
    }

    // 查找并与第一行交换
    fn search_and_view_at_beginning(&mut self, text: QString) {
        if let Some(index) = self
            .items()
            .iter()
            .position(|a| a.symbol.to_lower() == text.to_lower())
        {
            self.swap_row(0, index);
        }
    }
}
