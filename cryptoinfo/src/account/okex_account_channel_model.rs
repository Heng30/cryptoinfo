use super::data::okex::{res::AccountChannelDataDetial, AccountChannelItem as Item};
use crate::utility::Utility;
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
    }, members_qt: {
        total_eq: [QString; total_eq_changed], // 美金层面权益
        iso_eq: [QString; iso_eq_changed], // 美金层面逐仓仓位权益
        utime: [QString; utime_changed], // 更新时间
    }, signals_qt: {
    }, methods_qt: {
        set_item_qml: fn(&mut self),
        test_qml: fn(&self),
    }
);

impl Model {
    pub fn init(&mut self) { }

    pub fn set_account_state(&mut self, total_eq: String, iso_eq: String, utime: String) {
        self.total_eq = total_eq.into();
        self.iso_eq = iso_eq.into();
        self.utime = match utime.parse::<i64>() {
            Ok(v) => Utility::utc_seconds_to_local_string(v / 1000, "%H:%M:%S").into(),
            _ => "-".to_string().into(),
        };

        self.total_eq_changed();
        self.iso_eq_changed();
        self.utime_changed();
    }

    pub fn add_tmp_items(&mut self, raw_item: &Vec<AccountChannelDataDetial>) {
        let _ = self.mutex.lock().unwrap();

        self.tmp_items.clear();
        for item in raw_item {
            if item.eq_usd.parse::<f64>().map_err(|_| false).unwrap() < 1.0_f64 {
                continue;
            }
            self.tmp_items.push(Item {
                avail_eq: item.avail_eq.clone().into(),
                cash_bal: item.cash_bal.clone().into(),
                coin_usd_price: item.coin_usd_price.clone().into(),
                dis_eq: item.dis_eq.clone().into(),
                eq_usd: item.eq_usd.clone().into(),
                iso_eq: item.iso_eq.clone().into(),
                iso_upl: item.iso_upl.clone().into(),
                ccy: item.ccy.clone().into(),
                eq: item.eq.clone().into(),
                upl: item.upl.clone().into(),
                utime: match item.utime.parse::<i64>() {
                    Ok(v) => Utility::utc_seconds_to_local_string(v / 1000, "%H:%M:%S").into(),
                    _ => "-".to_string().into(),
                },
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
        for item in &qptr.borrow().tmp_items {
            qptr.borrow_mut().append(item.clone());
        }
    }

    fn test_qml(&self) {
        use super::OkexAccount;
        use crate::qobjmgr::{qobj, NodeType};
        let account = qobj::<OkexAccount>(NodeType::OkexAccount);
        let qptr = QBox::new(account);

        let path = "/home/blue/tmp/okex-account.json";
        if let Ok(msg) = std::fs::read_to_string(path) {
                OkexAccount::recv_pri_msg(qptr, msg);
        }
    }
}
