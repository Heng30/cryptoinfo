use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(QGadget, Clone, Default)]
pub struct MonitorItem {
    pub tx_hash: qt_property!(QString),
    pub blocktime: qt_property!(QString),
    pub from: qt_property!(QString),
    pub to: qt_property!(QString),
    pub tx_value: qt_property!(f64),
}

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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorEthDataResultRawItem {
    pub tx: String,
    pub timestamp: i64,
    pub from: String,
    pub to: String,

    #[serde(default)]
    pub amount: String,

    #[serde(default)]
    pub price: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorEthDataRawItem {
    pub result: Vec<MonitorEthDataResultRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MonitorEthRawItem {
    pub status: bool,
    pub data: MonitorEthDataRawItem,
}
