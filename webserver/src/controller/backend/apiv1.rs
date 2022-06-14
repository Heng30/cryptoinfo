use crate::response::apiv1;

#[get("/apiv1/coin/price")]
pub fn coint_price() -> apiv1::coin::Price {
    apiv1::coin::Price::new("/apiv1/coin/price")
}


