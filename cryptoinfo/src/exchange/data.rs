use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(QGadget, Clone, Default)]
pub struct ExchangeBtcItem {
    pub name: qt_property!(QString),
    pub balance: qt_property!(f64),
    pub income: qt_property!(f64),
    pub rate: qt_property!(f32),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ExchangeBtcDataExchangeRawItem {
    pub name: String,
    pub balance: f64,
    pub income: f64,
    pub rate: f32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ExchangeBtcDataRawItem {
    pub exchanges: Vec<ExchangeBtcDataExchangeRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ExchangeBtcRawItem {
    pub status: i32,
    pub data: ExchangeBtcDataRawItem,
}
