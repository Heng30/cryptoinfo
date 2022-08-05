pub use okex::req as okex_req;
pub use okex::res as okex_res;

pub mod okex {
    use qmetaobject::*;

    #[derive(Clone, Default, Debug)]
    pub struct SubscribeRawItem {
        pub channel: String,
        pub inst_type: String,
        pub is_pub: bool,
        pub is_ok: bool,
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct SubscribeItem {
        pub url: qt_property!(QString),
        pub channel: qt_property!(QString),
        pub inst_type: qt_property!(QString),
        pub is_ok: qt_property!(bool),
        pub is_pub: qt_property!(bool),
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct AccountChannelItem {
        pub avail_eq: qt_property!(QString),       // 可用保证金
        pub cash_bal: qt_property!(QString),       // 币种余额
        pub coin_usd_price: qt_property!(QString), // 币价
        pub dis_eq: qt_property!(QString),         // 美金层面币种折算权益
        pub eq_usd: qt_property!(QString),         // 币种权益美金价值
        pub iso_eq: qt_property!(QString),         // 币种逐仓仓位权益
        pub iso_upl: qt_property!(QString),        // 逐仓未实现盈亏
        pub utime: qt_property!(QString),          // 更新时间
        pub ccy: qt_property!(QString),            // 币种
        pub eq: qt_property!(QString),             // 币种总权益
        pub upl: qt_property!(QString),            // 未实现盈亏
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct PositionChannelItem {
        pub inst_type: qt_property!(QString),    // 产品类型
        pub inst_id: qt_property!(QString),      //	产品 ID，如 BTC-USD
        pub mgn_mode: qt_property!(QString),     //	保证金模式， cross：全仓 isolated：逐仓
        pub pos_side: qt_property!(QString),     //	持仓方向
        pub lever: qt_property!(QString),        // 杠杆倍数
        pub pos: qt_property!(QString),          //	持仓数量
        pub notional_usd: qt_property!(QString), //以美金价值为单位的持仓数量
        pub avg_px: qt_property!(QString),       // 开仓平均价
        pub mark_px: qt_property!(QString),      //	标记价格
        pub liq_px: qt_property!(QString),       // 预估强平价
        pub mgn_ratio: qt_property!(QString),    // 保证金率
        pub margin: qt_property!(QString),       // 保证金余额
        pub upl: qt_property!(QString),          // 未实现收益
        pub upl_ratio: qt_property!(QString),    // 未实现收益率
        pub ctime: qt_property!(QString),        // 持仓创建时间，Unix 时间戳的毫秒数格式
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct GreekChannelItem {
        pub ts: qt_property!(QString), // 更新时间
        pub ccy: qt_property!(QString),
        pub delta_bs: qt_property!(QString), // 币本位账户资产
    }

    #[derive(QGadget, Clone, Default, Debug)]
    pub struct MainAccountRestItem {
        pub avail_bal: qt_property!(QString),  // 可用余额
        pub frozen_bal: qt_property!(QString), // 冻结（不可用）
        pub ccy: qt_property!(QString),        // 币种
        pub bal: qt_property!(QString),        // 余额
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

        pub fn rest_header_sign(timestamp: &str, meth_url: &str, secret_key: &str) -> String {
            base64::encode(HMAC::mac(format!("{}{}", timestamp, meth_url), secret_key))
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
            #[serde(rename(serialize = "instType", deserialize = "instType"))]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inst_type: Option<String>, // 产品类型:
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct Subscribe {
            pub op: String,
            pub args: Vec<SubscribeArg>,
        }

        impl Subscribe {
            pub fn new(channel: &str, inst_type: &str) -> Self {
                Self {
                    op: "subscribe".to_string(),
                    args: vec![SubscribeArg {
                        channel: channel.to_string(),
                        inst_type: if inst_type.is_empty() {
                            None
                        } else {
                            Some(inst_type.to_string())
                        },
                    }],
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

        // 订阅channel返回信息
        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct MsgChannel {
            pub arg: ChannelArg,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct ChannelArg {
            pub channel: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct AccountChannel {
            pub arg: AccountChannelArg,
            pub data: Vec<AccountChannelData>,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct AccountChannelArg {
            pub channel: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct AccountChannelData {
            #[serde(rename(serialize = "totalEq", deserialize = "totalEq"))]
            pub total_eq: String, // 美金层面权益

            #[serde(rename(serialize = "isoEq", deserialize = "isoEq"))]
            pub iso_eq: String, // 美金层面逐仓仓位权益

            #[serde(rename(serialize = "uTime", deserialize = "uTime"))]
            pub utime: String, // 更新时间

            pub details: Vec<AccountChannelDataDetial>, // 各个币种的信息
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct AccountChannelDataDetial {
            #[serde(rename(serialize = "availEq", deserialize = "availEq"))]
            pub avail_eq: String, // 可用保证金

            #[serde(rename(serialize = "cashBal", deserialize = "cashBal"))]
            pub cash_bal: String, // 币种余额

            #[serde(rename(serialize = "coinUsdPrice", deserialize = "coinUsdPrice"))]
            pub coin_usd_price: String, // 币价

            #[serde(rename(serialize = "disEq", deserialize = "disEq"))]
            pub dis_eq: String, // 美金层面币种折算权益

            #[serde(rename(serialize = "eqUsd", deserialize = "eqUsd"))]
            pub eq_usd: String, // 币种权益美金价值

            #[serde(rename(serialize = "isoEq", deserialize = "isoEq"))]
            pub iso_eq: String, // 币种逐仓仓位权益

            #[serde(rename(serialize = "isoUpl", deserialize = "isoUpl"))]
            pub iso_upl: String, // 逐仓未实现盈亏

            #[serde(rename(serialize = "uTime", deserialize = "uTime"))]
            pub utime: String, // 更新时间

            pub ccy: String, // 币种
            pub eq: String,  // 币种总权益
            pub upl: String, // 未实现盈亏
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct PositionChannel {
            pub arg: PositionChannelArg,
            pub data: Vec<PositionChannelData>,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct PositionChannelArg {
            pub channel: String,
            #[serde(rename(serialize = "instType", deserialize = "instType"))]
            pub inst_type: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct PositionChannelData {
            #[serde(rename(serialize = "instType", deserialize = "instType"))]
            pub inst_type: String, // 产品类型
            #[serde(rename(serialize = "instId", deserialize = "instId"))]
            pub inst_id: String, //	产品 ID，如 BTC-USD-180216
            #[serde(rename(serialize = "mgnMode", deserialize = "mgnMode"))]
            pub mgn_mode: String, //	保证金模式， cross：全仓 isolated：逐仓
            #[serde(rename(serialize = "posSide", deserialize = "posSide"))]
            pub pos_side: String, //	持仓方向
            pub lever: String, // 杠杆倍数
            pub pos: String,   //	持仓数量
            #[serde(rename(serialize = "notionalUsd", deserialize = "notionalUsd"))]
            pub notional_usd: String, //以美金价值为单位的持仓数量
            #[serde(rename(serialize = "avgPx", deserialize = "avgPx"))]
            pub avg_px: String, // 开仓平均价
            #[serde(rename(serialize = "markPx", deserialize = "markPx"))]
            pub mark_px: String, //	标记价格
            #[serde(rename(serialize = "liqPx", deserialize = "liqPx"))]
            pub liq_px: String, // 预估强平价
            pub margin: String, // 保证金余额
            #[serde(rename(serialize = "mgnRatio", deserialize = "mgnRatio"))]
            pub mgn_ratio: String, // 保证金率
            pub upl: String,   // 未实现收益
            #[serde(rename(serialize = "uplRatio", deserialize = "uplRatio"))]
            pub upl_ratio: String, // 未实现收益率
            #[serde(rename(serialize = "cTime", deserialize = "cTime"))]
            pub ctime: String, // 持仓创建时间，Unix 时间戳的毫秒数格式
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct GreekChannel {
            pub arg: GreekChannelArg,
            pub data: Vec<GreekChannelData>,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct GreekChannelArg {
            pub channel: String,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct GreekChannelData {
            pub ccy: String,
            #[serde(rename(serialize = "deltaBS", deserialize = "deltaBS"))]
            pub delta_bs: String, // 币本位账户资产
            pub ts: String, // 更新时间
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct MainAccountRest {
            pub code: String,
            pub msg: String,
            pub data: Vec<MainAccountDataRest>,
        }

        #[derive(Serialize, Deserialize, Default, Debug)]
        pub struct MainAccountDataRest {
            #[serde(rename(serialize = "availBal", deserialize = "availBal"))]
            pub avail_bal: String, // 可用余额
            #[serde(rename(serialize = "frozenBal", deserialize = "frozenBal"))]
            pub frozen_bal: String, // 冻结（不可用）
            pub ccy: String, // 币种
            pub bal: String, // 余额
        }
    }
}
