pub use okex::req as okex_req;
pub use okex::res as okex_res;

pub mod okex {
    use qmetaobject::*;

    #[derive(Clone, Default, Debug)]
    pub struct SubscribeRawItem {
        pub channel: String,
        pub is_pub: bool,
        pub is_ok: bool,
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct SubscribeItem {
        pub url: qt_property!(QString),
        pub channel: qt_property!(QString),
        pub is_ok: qt_property!(bool),
        pub is_pub: qt_property!(bool),
    }

    pub mod req {
        #[allow(unused_imports)]
        use ::log::{debug, warn};
        use base64;
        use chrono::Local;
        use hmac_sha256::HMAC;
        use serde_derive::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct LoginApiMsg {
            #[serde(rename(serialize = "apiKey", deserialize = "apiKey"))]
            pub api_key: String,
            pub passphrase: String,
            pub timestamp: String,
            pub sign: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct LoginMsg {
            pub op: String,
            pub args: Vec<LoginApiMsg>,
        }

        impl LoginMsg {
            pub fn new(passphrase: &str, api_key: &str, secret_key: &str) -> String {
                let timestamp = format!("{}", Local::now().timestamp());
                let sign = base64::encode(HMAC::mac(
                    format!("{}GET/users/self/verify", &timestamp),
                    secret_key,
                ));
                let msg = Self {
                    op: "login".to_string(),
                    args: vec![LoginApiMsg {
                        api_key: api_key.to_string(),
                        passphrase: passphrase.to_string(),
                        timestamp: timestamp,
                        sign: sign,
                    }],
                };

                match serde_json::to_string(&msg) {
                    Ok(jstr) => return jstr,
                    Err(e) => debug!("{:?}", e),
                };
                return String::default();
            }
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct SubscribeArg {
            pub channel: String, // 频道名
            pub uly: String,     // 标的指数

            // 产品类型:
            //  SPOT：币币;
            //  MARGIN：币币杠杆
            //  SWAP：永续合约
            //  FUTURES：交割合约
            //  OPTION：期权
            //  ANY： 全部
            #[serde(rename(serialize = "instType", deserialize = "instType"))]
            pub inst_type: String,

            #[serde(rename(serialize = "instId", deserialize = "instId"))]
            pub inst_id: String, // 产品 ID
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct Subscribe {
            pub op: String,
            pub args: Vec<SubscribeArg>,
        }

        impl Subscribe {
            pub fn new(channel: &str) -> Self {
                Self {
                    op: "subscribe".to_string(),
                    args: vec![
                        SubscribeArg {
                            channel: channel.to_string(),
                            ..SubscribeArg::default()
                        }
                    ]
                }
            }

            pub fn to_json(&self) -> String {
                match serde_json::to_string(&self) {
                    Ok(jstr) => return jstr,
                    Err(e) => debug!("{:?}", e),
                };
                return "{}".to_string();
            }
        }
    }

    pub mod res {
        #[allow(unused_imports)]
        use ::log::{debug, warn};
        use serde_derive::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct MsgEvent {
            pub event: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct LoginMsg {
            pub event: String,
            pub msg: String,
            pub code: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct Subscribe {
            pub event: String,
            pub arg: SubscribeArg,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct SubscribeArg {
            pub channel: String,
        }
    }
}
