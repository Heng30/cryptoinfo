use crate::config::Config;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use std::env;
use tokio;
use webserver::controller::{backend, frontend};
use webserver::middleware::{cors::CORS, counter::Counter};

fn set_env(address: &str, port: u32) {
    env::set_var("ROCKET_PORT", format!("{}", port));
    env::set_var("ROCKET_ADDRESS", address);
}

pub fn init() {
    let config = qobj::<Config>(QNodeType::Config);
    if !config.enable_web_server {
        return;
    }

    set_env(
        &config.web_server_address.to_string(),
        config.web_server_port,
    );

    tokio::spawn(async move {
        rocket::ignite()
            .attach(Counter::new())
            .attach(CORS)
            .mount(
                "/",
                routes![
                    frontend::index::index,
                    frontend::index::index_html,
                    frontend::index::css,
                    frontend::index::js,
                    backend::api::counts,
                    backend::api::png,
                    backend::apiv1::private_data,
                    backend::apiv1::price
                ],
            )
            .launch();
    });
}
