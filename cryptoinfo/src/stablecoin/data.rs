use serde_derive::{Deserialize, Serialize};

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
