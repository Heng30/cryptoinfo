use crate::response::apiv1;

#[get("/apiv1/coin/price")]
pub fn coint_price() -> apiv1::coin::Price {
    apiv1::coin::Price::new("/apiv1/coin/price")
}

#[get("/apiv1/coin/btc-next-halving-day-left")]
pub fn bitcoin_next_halving_days_left() -> apiv1::coin::BTCNextHalving {
    apiv1::coin::BTCNextHalving::new("/apiv1/coin/btc-next-halving-day-left")
}

#[get("/apiv1/fear-greed")]
pub fn fear_greed() -> apiv1::fear_greed::FearGreed {
    apiv1::fear_greed::FearGreed::new("/apiv1/fear-greed")
}

#[get("/apiv1/market")]
pub fn market() -> apiv1::market::Market {
    apiv1::market::Market::new("/apiv1/market")
}
