use serde_derive::{Deserialize, Serialize};
use qmetaobject::*;

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

