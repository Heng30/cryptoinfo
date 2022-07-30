use super::data::{
    okex::{SubscribeItem as Item, SubscribeRawItem},
    okex_req,
};
use super::OkexAccount;
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

type RawItemVec = Vec<SubscribeRawItem>;

modeldata_struct!(Model, Item, members: {
    tmp_raw_items: RawItemVec,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
       set_item_qml: fn(&mut self),
    }
);

impl Model {
    pub fn init(&mut self) {
        let v: Vec<SubscribeRawItem> = vec![SubscribeRawItem {
            channel: "account".to_string(),
            is_pub: false,
            is_ok: false,
        }];
        for item in v {
            self.append(Item {
                channel: item.channel.into(),
                is_pub: item.is_pub,
                is_ok: item.is_ok,
                url: match item.is_pub {
                    true => "wss://ws.okx.com:8443/ws/v5/public".into(),
                    false => "wss://ws.okx.com:8443/ws/v5/private".into(),
                },
            });
        }
    }

    pub fn subscribe_only_channel(&self, account: &mut OkexAccount) {
        for item in self.items() {
            debug!("subscribe channel: {}", &item.channel);
            if item.is_pub {
                account.send_pub_msg(okex_req::Subscribe::new(&item.channel.to_string()).to_json());
            } else {
                account.send_pri_msg(okex_req::Subscribe::new(&item.channel.to_string()).to_json());
            }
        }
    }

    pub fn add_tmp_raw_item(&mut self, channel: String, is_ok: bool) {
        let _ = self.mutex.lock().unwrap();
        self.tmp_raw_items.push(SubscribeRawItem {
            channel,
            is_ok,
            ..Default::default()
        });
        self.updated();
    }

    pub fn offline(&mut self) {
        let mut v = vec![];
        let _ = self.mutex.lock().unwrap();
        for item in self.items().iter() {
            v.push(SubscribeRawItem {
                channel: item.channel.to_string(),
                is_ok: false,
                is_pub: item.is_pub,
            });
        }
        for item in v.into_iter() {
            self.tmp_raw_items.push(item);
        }

        self.updated();
    }

    fn set_item_qml(&mut self) {
        let mut v = vec![];
        let _ = self.mutex.lock().unwrap();
        for raw_item in &self.tmp_raw_items {
            for (i, item) in self.items().iter().enumerate() {
                if item.channel.to_string() == raw_item.channel {
                    let mut item = item.clone();
                    item.is_ok = raw_item.is_ok;
                    v.push((i, item));
                }
            }
        }

        for (i, item) in &v {
            self.set(*i, item.clone());
        }

        self.tmp_raw_items.clear();
    }
}
