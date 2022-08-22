use crate::qobjmgr::{qobj, NodeType as QNodeType};
use crate::translator::Translator;
#[allow(unused)]
use ::log::debug;
use ::log::warn;
use modeldata::*;
use platform_dirs::AppDirs;
use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};
use ContractStatsItem as Item;

#[derive(Serialize, Deserialize, Default, Debug)]
struct RawItem {
    ctype: String,
    win_lose_count: i32,
    float_value: f64,
}

#[derive(QGadget, Clone, Default)]
pub struct ContractStatsItem {
    ctype: qt_property!(QString),
    win_lose_count: qt_property!(i32),
    float_value: qt_property!(f64),
}

modeldata_struct!(Model, Item, members: {
        path: String,
    }, members_qt: {
        win_lose_counts: [i32; win_lose_counts_changed],
    }, signals_qt: {
    }, methods_qt: {
        save_qml: fn(&mut self),
        set_item_qml: fn(&mut self, index: usize, ctype: QString, win_lose_count: i32, float_value: f64),
    }
);

impl Model {
    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.path = app_dirs
            .data_dir
            .join("contract-stats.json")
            .to_str()
            .unwrap()
            .to_string();
        self.load();

        if self.items_len() != 4 {
            let translator = qobj::<Translator>(QNodeType::Translator);
            let ctypes = vec![
                translator.tr("盈利小于100%".to_string().into()),
                translator.tr("盈利大于100%".to_string().into()),
                translator.tr("亏损小于50%".to_string().into()),
                translator.tr("亏损大于50%".to_string().into()),
            ];

            self.clear();
            for ctype in ctypes {
                self.append(Item {
                    ctype,
                    ..Default::default()
                });
            }
        }

        self.cal_win_lose_counts();
    }

    fn cal_win_lose_counts(&mut self) {
        let mut win_lose_counts = 0;
        for item in self.items().iter() {
            win_lose_counts += item.win_lose_count;
        }
        self.win_lose_counts = win_lose_counts;
        self.win_lose_counts_changed();
    }

    pub fn load(&mut self) {
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if let Ok(raw_items) = serde_json::from_str::<Vec<RawItem>>(&text) {
                for item in raw_items {
                    self.append(Item {
                        ctype: item.ctype.into(),
                        win_lose_count: item.win_lose_count,
                        float_value: item.float_value,
                    });
                }
            }
        }
    }

    fn save_qml(&mut self) {
        let mut raw_items = vec![];
        for item in self.items() {
            raw_items.push(RawItem {
                ctype: item.ctype.to_string(),
                win_lose_count: item.win_lose_count,
                float_value: item.float_value,
            });
        }

        match serde_json::to_string_pretty(&raw_items) {
            Ok(text) => {
                if let Err(_) = std::fs::write(&self.path, text) {
                    warn!("save {:?} failed", &self.path);
                }
            }
            Err(e) => warn!("{:?}", e),
        }
    }

    fn set_item_qml(
        &mut self,
        index: usize,
        ctype: QString,
        win_lose_count: i32,
        float_value: f64,
    ) {
        self.set(
            index,
            Item {
                ctype: ctype,
                win_lose_count: win_lose_count,
                float_value: float_value,
            },
        );
        self.cal_win_lose_counts();
    }
}
