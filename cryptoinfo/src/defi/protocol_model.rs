use super::data::{ProtocolItem as Item, RawProtocolItem as RawItem};
use super::sort::{ProtocolSortKey as SortKey, SortDir};
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::utility;
#[allow(unused_imports)]
use ::log::{debug, warn};
use cstr::cstr;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use std::cmp::Ordering;

modeldata_struct!(Model, Item, members: {
        path: String,
        sort_key: u32,
        sort_dir: SortDir,
        url: String,
    }, members_qt: {
        text: [QString; text_changed],
        bull_percent: [f32; bull_percent_changed], // 上涨占比
        update_now: [bool; update_now_changed], // 马上更新
        update_time: [QString; update_time_changed],// 数据更新时间
    }, signals_qt: {
    }, methods_qt: {
        update_all_qml: fn(&mut self),
        sort_by_key_qml: fn(&mut self, key: u32),
        toggle_sort_dir_qml: fn(&mut self),
        search_and_view_at_beginning_qml: fn(&mut self, text: QString),
    }
);

impl Model {
    pub fn init(&mut self) {
        qml_register_enum::<SortKey>(
            cstr!("DefiProtocolSortKey"),
            1,
            0,
            cstr!("DefiProtocolSortKey"),
        );

        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        self.sort_key = SortKey::Index as u32;
        self.update_now = false;
        self.url = "https://api.llama.fi/protocols".to_string();

        self.path = app_dirs
            .data_dir
            .join("defi-protocols.json")
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
            self.sort_by_key_qml(self.sort_key);
        }
    }

    // 缓存数据到本地
    fn save(&self, text: &str) {
        if let Err(_) = std::fs::write(&self.path, text) {
            warn!("write to {} error", &self.path);
        }
    }

    // 更新model
    fn update_all_qml(&mut self) {
        if self.text.is_empty() {
            return;
        }

        let text = self.text.to_string();
        self.reset(&text);
        self.save(&text);
        self.sort_by_key_qml(self.sort_key);
        self.update_time = utility::Utility::default().local_time_now_qml(QString::from("%H:%M:%S"));
        self.update_time_changed();
    }

    // 更新数据
    pub fn update_text(&mut self, text: String) {
        self.text = text.into();
        self.text_changed();
    }

    // 设置反向搜索
    fn toggle_sort_dir_qml(&mut self) {
        match self.sort_dir {
            SortDir::UP => self.sort_dir = SortDir::DOWN,
            _ => self.sort_dir = SortDir::UP,
        }
    }

    // 跟据key进行搜索
    fn sort_by_key_qml(&mut self, key: u32) {
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
                } else if key == SortKey::Per1H {
                    self.items_mut().sort_by(|a, b| {
                        a.percent_change_1h
                            .partial_cmp(&b.percent_change_1h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per24H {
                    self.items_mut().sort_by(|a, b| {
                        a.percent_change_24h
                            .partial_cmp(&b.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per7D {
                    self.items_mut().sort_by(|a, b| {
                        a.percent_change_7d
                            .partial_cmp(&b.percent_change_7d)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::MarketCap {
                    self.items_mut().sort_by(|a, b| {
                        a.market_cap_usd
                            .partial_cmp(&b.market_cap_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Staking {
                    self.items_mut().sort_by(|a, b| {
                        a.staking.partial_cmp(&b.staking).unwrap_or(Ordering::Less)
                    });
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
                } else if key == SortKey::Per1H {
                    self.items_mut().sort_by(|a, b| {
                        b.percent_change_1h
                            .partial_cmp(&a.percent_change_1h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per24H {
                    self.items_mut().sort_by(|a, b| {
                        b.percent_change_24h
                            .partial_cmp(&a.percent_change_24h)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::Per7D {
                    self.items_mut().sort_by(|a, b| {
                        b.percent_change_7d
                            .partial_cmp(&a.percent_change_7d)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::MarketCap {
                    self.items_mut().sort_by(|a, b| {
                        b.market_cap_usd
                            .partial_cmp(&a.market_cap_usd)
                            .unwrap_or(Ordering::Less)
                    });
                } else if key == SortKey::TVL {
                    self.items_mut()
                        .sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(Ordering::Less));
                } else if key == SortKey::Staking {
                    self.items_mut().sort_by(|a, b| {
                        b.staking.partial_cmp(&a.staking).unwrap_or(Ordering::Less)
                    });
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
            symbol: raw_item.symbol.clone().into(),
            tvl: raw_item.tvl,
            market_cap_usd: raw_item.mcap,
            staking: raw_item.staking,
            percent_change_1h: raw_item.change_1h.unwrap_or(0.0),
            percent_change_24h: raw_item.change_1d.unwrap_or(0.0),
            percent_change_7d: raw_item.change_7d.unwrap_or(0.0),
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
        let raw_item: Vec<RawItem> = serde_json::from_str(&text).unwrap_or(vec![]);
        let mut bull_count = 0;
        let mut bear_count = 0;

        for (i, item) in raw_item.iter().enumerate() {
            if i >= 100 {
                break;
            }

            if item.change_1d.unwrap_or(0.0) > 0.0 {
                bull_count += 1;
            } else {
                bear_count += 1;
            }

            if self.items_len() <= i {
                self.add_item(i, &item);
            } else {
                self.set_item(i, &item);
            }
        }

        self.bull_percent = bull_count as f32 / (bull_count + bear_count) as f32;
        self.bull_percent_changed();
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
