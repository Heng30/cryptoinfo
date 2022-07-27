use serde_derive::{Deserialize, Serialize};

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