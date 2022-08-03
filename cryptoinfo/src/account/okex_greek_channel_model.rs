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
        avg_value: [f64; avg_value_changed], // 平均价值
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
            if item.delta_bs.parse::<f64>().unwrap_or(0_f64) < 0.0001 {
                continue;
            }

            self.tmp_items.push(Item {
                ccy: item.ccy.clone().into(),
                delta_bs: item.delta_bs.clone().into(),
                delta_pa: item.delta_pa.clone().into(),
                ts: Utility::utc_seconds_to_local_string(item.ts.parse::<i64>().unwrap_or(0) / 1000, "%m-%d %H:%M").into(),
            });
        }
        self.updated();
    }

    fn set_item_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        if self.tmp_items.is_empty() {
            return;
        }
        self.clear();

        let qptr = QBox::new(self);
        if qptr.borrow().items_len() <= 0 {
            for item in &qptr.borrow().tmp_items {
                qptr.borrow_mut().append(item.clone());
            }
        } else {
            for item in &qptr.borrow().tmp_items {
                for (index, it) in qptr.borrow().items().iter().enumerate() {
                    if it.ccy == item.ccy {
                        qptr.borrow_mut().set(index, item.clone());
                    } else {
                        qptr.borrow_mut().append(item.clone());
                    }
                }
            }
        }

        for item in qptr.borrow().items() {
            qptr.borrow_mut().avg_value += item.delta_pa.to_string().parse::<f64>().unwrap_or(0_f64);
        }

        self.avg_value = if self.avg_value <= 0_f64 || self.items_len() <= 0 {
            0_f64
        } else {
            self.avg_value / self.items_len() as f64
        };
        self.avg_value_changed();

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
