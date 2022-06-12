use crate::response::coin;

#[get("/counts")]
pub fn counts() {
}

#[get("/coin/price")]
pub fn coint_price() -> coin::price::Price {
    coin::price::Price{}
}
