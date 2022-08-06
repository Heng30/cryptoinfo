use super::data::{
    okex:: WithdrawalRestItem as Item,
    okex_res::{WithdrawalDataRest, WithdrawalRest as RawItem},
};
use crate::httpclient;
use crate::utility::Utility;
use ::log::debug;
use modeldata::*;
use qmetaobject::*;

type ItemVec = Vec<Item>;

modeldata_struct!(Model, Item, members: {
        tmp_items: ItemVec,
        url: String,
        path: String,
    }, members_qt: {
        update_now: [bool; update_now_changed],
        update_time: [QString; update_time_changed],
    }, signals_qt: {
    }, methods_qt: {
        refresh_qml: fn(&mut self),
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
        let _ = self.borrow_mut().mutex.lock().unwrap();
        return self.borrow().update_now;
    }

    fn disable_update_now(&self) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().update_now = false;
    }

    fn parse_body(&mut self, text: &str) {
        let _ = self.borrow_mut().mutex.lock().unwrap();
        self.borrow_mut().cache_items(text);
    }
}

impl httpclient::OkexDownloadProvider for QBox<Model> {
    fn path(&self) -> String {
        return self.borrow().path.clone();
    }
}

impl Model {
    pub fn init(&mut self) {
        self.path = "/api/v5/asset/withdrawal-history".to_string();
        self.url = format!("{}{}", "https://aws.okx.com", self.path);
        self.async_update_model();
    }

    pub fn refresh_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        self.update_now = true;
    }

    fn new_item(raw_item: &WithdrawalDataRest) -> Item {
        return Item {
            ccy: raw_item.ccy.clone().into(),
            tx_id: raw_item.tx_id.clone().into(),
            from: raw_item.from.clone().into(),
            to: raw_item.to.clone().into(),
            chain: raw_item.chain.clone().into(),
            amt: raw_item.amt.clone().into(),
            state: raw_item.state.clone().into(),
            fee: raw_item.fee.clone().into(),
            ts: Utility::utc_seconds_to_local_string(
                raw_item.ts.parse::<i64>().unwrap_or(0) / 1000,
                "%y-%m-%d %H:%M",
            ).into(),
        };
    }

    fn update_model(&mut self, _text: String) {
        {
            let _ = self.mutex.lock().unwrap();
            self.clear();
            let qptr = QBox::new(self);
            for item in qptr.borrow().tmp_items.iter() {
                self.append(item.clone());
            }
        }

        self.update_time = Utility::local_time_now("%H:%M:%S").into();
        self.update_time_changed();
    }

    fn async_update_model(&mut self) {
        let qptr = QBox::new(self);
        let cb = qmetaobject::queued_callback(move |text: String| {
            qptr.borrow_mut().update_model(text);
        });

        httpclient::download_timer_okex_pro(qptr, 1, cb);
    }

    fn cache_items(&mut self, text: &str) {
        match serde_json::from_str::<RawItem>(text) {
            Ok(raw_item) => {
                if raw_item.data.is_empty() {
                    return;
                }
                self.tmp_items.clear();

                for item in raw_item.data.iter() {
                    self.tmp_items.push(Self::new_item(&item));
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }
}
