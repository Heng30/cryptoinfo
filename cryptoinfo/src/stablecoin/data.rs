use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[derive(QGadget, Clone, Default)]
pub struct StableCoinChainItem {
    pub index: qt_property!(u32),
    pub name: qt_property!(QString),
    pub symbol: qt_property!(QString),
    pub circulating: qt_property!(f64),
}

#[derive(QGadget, Clone, Default)]
pub struct StableCoinMcapItem {
    pub index: qt_property!(u32),
    pub name: qt_property!(QString),
    pub symbol: qt_property!(QString),
    pub peg_type: qt_property!(QString),
    pub price_source: qt_property!(QString),
    pub circulating: qt_property!(f64),
    pub price: qt_property!(f64),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StableCoinAssetCirculatingRawItem {
    #[serde(default, rename(serialize = "peggedUSD", deserialize = "peggedUSD"))]
    pub usd: f64,

    #[serde(default, rename(serialize = "peggedEUR", deserialize = "peggedEUR"))]
    pub eur: f64,

    #[serde(default, rename(serialize = "peggedVAR", deserialize = "peggedVAR"))]
    pub var: f64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StableCoinAssetRawItem {
    pub name: String,
    pub symbol: String,
    pub circulating: StableCoinAssetCirculatingRawItem,

    #[serde(rename(serialize = "pegType", deserialize = "pegType"))]
    pub peg_type: String,

    #[serde(rename(serialize = "priceSource", deserialize = "priceSource"))]
    pub price_source: Option<String>,

    pub price: Option<f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StableCoinRawItem {
    #[serde(rename(serialize = "peggedAssets", deserialize = "peggedAssets"))]
    pub pegged_assets: Vec<StableCoinAssetRawItem>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StableCoinChainCirculatingRawItem {
    #[serde(default, rename(serialize = "peggedUSD", deserialize = "peggedUSD"))]
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct StableCoinChainRawItem {
    #[serde(rename(serialize = "totalCirculatingUSD", deserialize = "totalCirculatingUSD"))]
    pub circulating: StableCoinChainCirculatingRawItem,

    #[serde(rename(serialize = "tokenSymbol", deserialize = "tokenSymbol"))]
    pub symbol: Option<String>,
    pub name: String,
}
