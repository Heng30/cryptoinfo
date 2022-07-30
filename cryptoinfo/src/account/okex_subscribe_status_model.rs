use super::data::okex::{SubscribeItem as Item, SubscribeRawItem };
use modeldata::*;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

modeldata_struct!(Model, Item, members: {
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
    }
);

impl Model {
    pub fn init(&mut self) {
        let v: Vec<SubscribeRawItem> = vec![];
        for item in v {
            self.append(Item {
                channel: item.channel.into(),
                is_pub: item.is_pub,
                is_ok: false,
                url: match item.is_pub {
                    true => "wss://ws.okx.com:8443/ws/v5/public".into(),
                    false => "wss://ws.okx.com:8443/ws/v5/prive".into(),
                }
            })
        }
    }

    pub fn set_item(&mut self, channel: String, is_ok: bool) {
        let mut tmp = Item::default();
        let mut idx = 0;
        for (i, item) in self.items().iter().enumerate() {
            if item.channel.to_string() == channel {
                tmp = item.clone();
                tmp.is_ok = is_ok;
                idx = i;
            }
        }

        if tmp.is_ok {
            self.set(idx, tmp);
        }
    }
}
