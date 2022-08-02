use super::data::okex::{res::PositionChannelData, PositionChannelItem as Item};
use crate::utility::Utility;
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
    }, members_qt: {
        total_eq: [f64; total_eq_changed], // 总仓位(美元)
        iso_eq: [f64; iso_eq_changed], // 未实现盈亏(美元)
        utime: [QString; utime_changed], // 更新时间
    }, signals_qt: {
    }, methods_qt: {
        set_item_qml: fn(&mut self),
        test_qml: fn(&self),
    }
);

impl Model {
    pub fn init(&mut self) {}

    pub fn add_tmp_items(&mut self, raw_item: &Vec<PositionChannelData>) {
        let _ = self.mutex.lock().unwrap();

        self.tmp_items.clear();
        for item in raw_item {
            if item.inst_type.to_uppercase() != "SWAP" {
                continue;
            }

            self.tmp_items.push(Item {
                inst_type: item.inst_type.clone().into(),
                mgn_mode: item.mgn_mode.clone().into(),
                lever: item.lever.clone().into(),
                pos_side: item.pos_side.clone().into(),
                pos: item.pos.clone().into(),
                notional_usd: item.notional_usd.clone().into(),
                avg_px: item.avg_px.clone().into(),
                mark_px: item.mark_px.clone().into(),
                liq_px: item.liq_px.clone().into(),
                margin: item.margin.clone().into(),
                mgn_ratio: item.mgn_ratio.clone().into(),
                upl: item.upl.clone().into(),
                upl_ratio: item.upl_ratio.clone().into(),
                inst_id: match item.inst_id.rsplit_once('-') {
                    Some(res) => res.0.clone().into(),
                    _ => "-".to_string().into(),
                },
                ctime: match item.ctime.parse::<i64>() {
                    Ok(v) => Utility::utc_seconds_to_local_string(v / 1000, "%m-%d %H:%M").into(),
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
        self.total_eq = 0_f64;
        self.iso_eq = 0_f64;

        let qptr = QBox::new(self);
        for item in &qptr.borrow().tmp_items {
            qptr.borrow_mut().total_eq += item.margin.to_string().parse::<f64>().unwrap_or(0_f64);
            qptr.borrow_mut().iso_eq += item.upl.to_string().parse::<f64>().unwrap_or(0_f64);
            qptr.borrow_mut().append(item.clone());
        }

        self.utime = Utility::local_time_now("%H:%M:%S").into();
        self.total_eq_changed();
        self.iso_eq_changed();
        self.utime_changed();
    }

    fn test_qml(&self) {
        use super::OkexAccount;
        use crate::qobjmgr::{qobj, NodeType};
        let account = qobj::<OkexAccount>(NodeType::OkexAccount);
        let qptr = QBox::new(account);

        let path = "/home/blue/tmp/okex-position.json";
        if let Ok(msg) = std::fs::read_to_string(path) {
            OkexAccount::recv_pri_msg(qptr, msg);
        }
    }
}
