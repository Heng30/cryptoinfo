use super::data::okex::{res::GreekChannelData, GreekChannelItem as Item};
use crate::utility::Utility;
use modeldata::*;
use qmetaobject::*;
use std::sync::Mutex;

type ItemVec = Mutex<Option<Vec<Item>>>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
    }, members_qt: {
        utime: [QString; utime_changed], // 更新时间
    }, signals_qt: {
    }, methods_qt: {
        set_item_qml: fn(&mut self),
        test_qml: fn(&self),
    }
);

impl Model {
    pub fn init(&mut self) {}

    pub fn add_tmp_items(&mut self, raw_item: &Vec<GreekChannelData>) {
        let mut tmp_items = self.tmp_items.lock().unwrap();
        *tmp_items = Some(vec![]);

        for item in raw_item {
            tmp_items.as_mut().unwrap().push(Item {
                ccy: item.ccy.clone().into(),
                delta_bs: item.delta_bs.clone().into(),
                ts: Utility::utc_seconds_to_local_string(
                    item.ts.parse::<i64>().unwrap_or(0) / 1000,
                    "%m-%d %H:%M",
                )
                .into(),
            });
        }
        self.updated();
    }

    fn set_item_qml(&mut self) {
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        if self.items_len() <= 0 {
            for item in tmp_items.unwrap() {
                if item.delta_bs.to_string().parse::<f64>().unwrap_or(0_f64) >= 0.0001 {
                    self.append(item);
                }
            }
        } else {
            for item in tmp_items.unwrap() {
                let mut found = false;
                let is_small_count =
                    item.delta_bs.to_string().parse::<f64>().unwrap_or(0_f64) < 0.0001;
                for (index, it) in self.items().iter().enumerate() {
                    if it.ccy == item.ccy {
                        if is_small_count {
                            self.remove_rows(index, 1);
                        } else {
                            self.set(index, item.clone());
                        }
                        found = true;
                        break;
                    }
                }

                if !found && !is_small_count {
                    self.append(item);
                }
            }
        }

        self.utime = Utility::local_time_now("%H:%M:%S").into();
        self.utime_changed();
    }

    fn test_qml(&self) {
        use super::OkexAccount;
        use crate::qobjmgr::{qobj, NodeType};
        let account = qobj::<OkexAccount>(NodeType::OkexAccount);
        let qptr = QBox::new(account);

        let path = "/home/blue/tmp/okex-greek.json";
        if let Ok(msg) = std::fs::read_to_string(path) {
            OkexAccount::recv_pri_msg(qptr, msg);
        }
    }
}
