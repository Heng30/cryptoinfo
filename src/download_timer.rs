use std::cell::RefCell;
use tokio::{self, time};

use qmetaobject::prelude::*;
use qmetaobject::{QObjectPinned, QUrl};
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::pricer::Model as pricer_model;

pub async fn price_update_timer(pricer_model: Arc<RefCell<pricer_model>>) {
    let mut interval = time::interval(time::Duration::from_secs(1));

    loop {
        // println!("{}", Local::now().format("%F %T").to_string());
        pricer_model.borrow_mut().foo();
        interval.tick().await;
    }
}
