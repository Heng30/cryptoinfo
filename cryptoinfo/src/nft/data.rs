use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTGemDataStatRawItem {
    pub one_day_volume: f64,
    pub one_day_change: f64,
    pub seven_day_change: f64,
    pub total_volume: f64,
    pub total_sales: f64,
    pub total_supply: f64,
    pub num_owners: f64,
    pub floor_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTGemDataRawItem {
    pub name: String,
    pub stats: NFTGemDataStatRawItem,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTGemRawItem {
    pub data: Vec<NFTGemDataRawItem>,
}

#[derive(QGadget, Clone, Default)]
pub struct NTFGemItem {
    pub name: qt_property!(QString),
    pub one_day_volume: qt_property!(f64),
    pub one_day_change: qt_property!(f64),
    pub seven_day_change: qt_property!(f64),
    pub total_volume: qt_property!(f64),
    pub total_sales: qt_property!(f64),
    pub total_supply: qt_property!(f64),
    pub num_owners: qt_property!(f64),
    pub floor_price: qt_property!(f64),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTGenieRawItem {
    pub name: String,
    pub address: String,
    pub floor: f64,
    pub volume: f64,
    pub owners: u32,
    pub supply: u32,

    #[serde(rename(serialize = "volumeChange", deserialize = "volumeChange"))]
    pub volume_change: f64,

    #[serde(rename(serialize = "marketCap", deserialize = "marketCap"))]
    pub market_cap: f64,

    #[serde(rename(serialize = "percentListed", deserialize = "percentListed"))]
    pub percent_listed: String,
}

#[derive(QGadget, Clone, Default)]
pub struct NTFGenieItem {
    pub name: qt_property!(QString),
    pub address: qt_property!(QString),
    pub percent_listed: qt_property!(QString),
    pub volume: qt_property!(f64),
    pub volume_change: qt_property!(f64),
    pub floor: qt_property!(f64),
    pub market_cap: qt_property!(f64),
    pub owners: qt_property!(u32),
    pub supply: qt_property!(u32),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTSudoSwapRawItem {
    pub collections: Vec<NFTSudoSwapCollectionRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NFTSudoSwapCollectionRawItem {
    pub address: String,
    pub name: Option<String>,
    pub buy_quote: f64,
    pub offer_tvl: f64,
    pub sell_quote: f64,
    pub pool_count: i32,
    pub item_count: i32,
}

#[derive(QGadget, Clone, Default)]
pub struct NTFSudoSwapItem {
    pub address: qt_property!(QString),
    pub name: qt_property!(QString),
    pub buy_quote: qt_property!(f64),
    pub sell_quote: qt_property!(f64),
    pub offer_tvl: qt_property!(f64),
    pub pool_count: qt_property!(i32),
    pub item_count: qt_property!(i32),
}
