use super::data::{
    okex::{SubscribeItem as Item, SubscribeRawItem},
    okex_req,
};
use super::OkexAccount;
use ::log::debug;
use modeldata::*;
use qmetaobject::*;
use std::sync::Mutex;

type RawItemVec = Mutex<Option<Vec<SubscribeRawItem>>>;

modeldata_struct!(Model, Item, members: {
    tmp_items: RawItemVec,
    }, members_qt: {
    }, signals_qt: {
    }, methods_qt: {
       set_item_qml: fn(&mut self),
    }
);

impl Model {
    pub fn init(&mut self) {
        let v: Vec<SubscribeRawItem> = vec![
            SubscribeRawItem {
                channel: "account".to_string(),
                inst_type: "".to_string(),
                is_pub: false,
                is_ok: false,
            },
            SubscribeRawItem {
                channel: "positions".to_string(),
                inst_type: "SWAP".to_string(),
                is_pub: false,
                is_ok: false,
            },
            SubscribeRawItem {
                channel: "account-greeks".to_string(),
                inst_type: "".to_string(),
                is_pub: false,
                is_ok: false,
            },
        ];
        for item in v {
            self.append(Item {
                channel: item.channel.into(),
                inst_type: item.inst_type.into(),
                is_pub: item.is_pub,
                is_ok: item.is_ok,
                url: match item.is_pub {
                    true => "wss://ws.okx.com:8443/ws/v5/public".into(),
                    false => "wss://ws.okx.com:8443/ws/v5/private".into(),
                },
            });
        }
    }

    pub fn subscribe_channel(&self, account: &mut OkexAccount) {
        for item in self.items() {
            debug!("subscribe channel: {}", &item.channel);
            if item.is_pub {
                account.send_pub_msg(
                    okex_req::Subscribe::new(
                        &item.channel.to_string(),
                        &item.inst_type.to_string(),
                    )
                    .to_json(),
                );
            } else {
                account.send_pri_msg(
                    okex_req::Subscribe::new(
                        &item.channel.to_string(),
                        &item.inst_type.to_string(),
                    )
                    .to_json(),
                );
            }
        }
    }

    pub fn add_tmp_raw_item(&mut self, channel: String, is_ok: bool) {
        let mut items = self.tmp_items.lock().unwrap();
        if items.is_none() {
            *items = Some(vec![]);
        }

        items.as_mut().unwrap().push(SubscribeRawItem {
            channel,
            is_ok,
            ..Default::default()
        });
        self.updated();
    }

    pub fn offline(&mut self) {
        let mut v = vec![];
        for item in self.items().iter() {
            v.push(SubscribeRawItem {
                channel: item.channel.to_string(),
                inst_type: item.inst_type.to_string(),
                is_ok: false,
                is_pub: item.is_pub,
            });
        }
        {
            let mut items = self.tmp_items.lock().unwrap();
            if items.is_none() {
                *items = Some(vec![]);
            }
            for item in v {
                items.as_mut().unwrap().push(item);
            }
        }

        self.updated();
    }

    fn set_item_qml(&mut self) {
        let mut v = vec![];
        let tmp_items = self.tmp_items.lock().unwrap().take();
        if tmp_items.is_none() {
            return;
        }

        for titem in tmp_items.unwrap() {
            for (i, item) in self.items().iter().enumerate() {
                if item.channel.to_string() == titem.channel {
                    let mut item = item.clone();
                    item.is_ok = titem.is_ok;
                    v.push((i, item));
                    break;
                }
            }
        }

        for (i, item) in v {
            self.set(i, item);
        }
    }
}
