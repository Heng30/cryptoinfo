use super::data::okex::{res::GreekChannelData, GreekChannelItem as Item};
use crate::utility::Utility;
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

type ItemVec = Vec<Item>;

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
        let _ = self.mutex.lock().unwrap();

        self.tmp_items.clear();
        for item in raw_item {
            self.tmp_items.push(Item {
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
        let _ = self.mutex.lock().unwrap();
        if self.tmp_items.is_empty() {
            return;
        }

        let qptr = QBox::new(self);
        if qptr.borrow().items_len() <= 0 {
            for item in &qptr.borrow().tmp_items {
                if item.delta_bs.to_string().parse::<f64>().unwrap_or(0_f64) >= 0.0001 {
                    qptr.borrow_mut().append(item.clone());
                }
            }
        } else {
            for item in &qptr.borrow().tmp_items {
                let mut found = false;

                let is_small_count =
                    item.delta_bs.to_string().parse::<f64>().unwrap_or(0_f64) < 0.0001;
                for (index, it) in qptr.borrow().items().iter().enumerate() {
                    if it.ccy == item.ccy {
                        if is_small_count {
                            qptr.borrow_mut().remove_rows(index, 1);
                        } else {
                            qptr.borrow_mut().set(index, item.clone());
                        }
                        found = true;
                        break;
                    }
                }

                if !found && !is_small_count {
                    qptr.borrow_mut().append(item.clone());
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
