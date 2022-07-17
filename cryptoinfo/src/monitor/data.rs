use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorBtcDataHitRawItem {
    #[serde(rename(serialize = "txHash", deserialize = "txHash"))]
    pub tx_hash: String,

    #[serde(rename(serialize = "txValue", deserialize = "txValue"))]
    pub tx_value: f64,

    pub blocktime: String,
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorBtcDataRawItem {
    pub total: f64,
    pub hits: Vec<MonitorBtcDataHitRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorBtcRawItem {
    pub code: i32,
    pub data: MonitorBtcDataRawItem,
}
