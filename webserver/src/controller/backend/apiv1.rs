use crate::response::apiv1;

#[get("/apiv1/private.json")]
pub fn private_data() -> apiv1::Private {
    apiv1::Private::new("/apiv1/private.json")
}

#[get("/apiv1/price.json")]
pub fn price() -> apiv1::Price {
    apiv1::Price::new("/apiv1/price.json")
}
