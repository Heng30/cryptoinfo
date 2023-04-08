use crate::config::Config;
use crate::qobjmgr::{qobj, NodeType as QNodeType};
use rocket::config::{Config as RConfig, Environment, LoggingLevel};
use webserver::controller::{backend, frontend};
use webserver::middleware::{cors::CORS, counter::Counter};

pub fn init() {
    let config = qobj::<Config>(QNodeType::Config);
    if !config.enable_web_server {
        return;
    }

    let config = RConfig::build(Environment::Production)
        .address(&config.web_server_address.to_string())
        .port(config.web_server_port as u16)
        .log_level(LoggingLevel::Critical)
        .finalize()
        .unwrap();

    tokio::spawn(async move {
        rocket::custom(config)
            .attach(Counter::new())
            .attach(CORS)
            .mount(
                "/",
                routes![
                    frontend::index::index,
                    frontend::index::index_html,
                    frontend::index::css,
                    frontend::index::js,
                    backend::api::png,
                    backend::apiv1::private_data,
                    backend::apiv1::price
                ],
            )
            .launch();
    });
}
