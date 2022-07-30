#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use chrono::Local;
use env_logger::fmt::Color as LColor;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use qmetaobject::prelude::*;
use qmetaobject::QUrl;
use std::io::Write;
use tokio;

mod config;
mod database;
mod chain;
mod chart;
mod ghotkey;
mod price;
mod qobjmgr;
mod res;
mod tool;
mod translator;
mod utility;
mod websvr;
mod news;
mod version;
mod httpclient;
mod exchange;
mod monitor;
mod stablecoin;
mod address;
mod account;

#[tokio::main]
async fn main() {
    init_logger();

    debug!("{}", "start...");

    res::resource_init();
    let mut engine = QmlEngine::new();

    let _app_dir = qobjmgr::init_app_dir();
    let _utility = qobjmgr::init_utility(&mut engine);
    let _config = qobjmgr::init_config(&mut engine);
    let _pidlock = qobjmgr::init_pidlock();
    let _login_table = qobjmgr::init_login_table(&mut engine);
    let _hotkey = qobjmgr::init_hotkey(&mut engine);
    let _translator = qobjmgr::init_translator(&mut engine);
    let _encipher = qobjmgr::init_encipher(&mut engine);
    let _addrbook_model = qobjmgr::init_addrbook_model(&mut engine);
    let _handbook_model = qobjmgr::init_handbook_model(&mut engine);
    let _fundbook_model = qobjmgr::init_fundbook_model(&mut engine);
    let _bookmark_model = qobjmgr::init_bookmark_model(&mut engine);
    let _todo_model = qobjmgr::init_todo_model(&mut engine);
    let _note = qobjmgr::init_note(&mut engine);
    let _price_model = qobjmgr::init_price_model(&mut engine);
    let _price_addition = qobjmgr::init_price_addition(&mut engine);
    let _defi_protocol_model = qobjmgr::init_chain_protocol_model(&mut engine);
    let _defi_chain_model = qobjmgr::init_chain_tvl_model(&mut engine);
    let _defi_chain_name_model = qobjmgr::init_chain_name_model(&mut engine);
    let _defi_chain_tvl_model = qobjmgr::init_chart_chain_tvl_model(&mut engine);
    let _news = qobjmgr::init_news_model(&mut engine);
    let _exchange_btc_model = qobjmgr::init_exchange_btc_model(&mut engine);
    let _monitor_btc_model = qobjmgr::init_monitor_btc_model(&mut engine);
    let _monitor_eth_model = qobjmgr::init_monitor_eth_model(&mut engine);
    let _stable_coin_mcap_model = qobjmgr::init_stable_coin_mcap_model(&mut engine);
    let _stable_coin_chain_model = qobjmgr::init_stable_coin_chain_model(&mut engine);
    let _chain_yield_model = qobjmgr::init_chain_yield_model(&mut engine);
    let _address_eth_model = qobjmgr::init_address_eth_model(&mut engine);
    let _chain_eth_token_model = qobjmgr::init_chain_eth_token_model(&mut engine);
    let _okex_account = qobjmgr::init_okex_account(&mut engine);
    let _okex_sub_sta_model = qobjmgr::init_okex_subscribe_status_model(&mut engine);


    websvr::start();

    engine.load_url(QUrl::from(QString::from("qrc:/res/qml/main.qml")));
    engine.exec();

    // 保证UI部分先被析构
    drop(engine);

    debug!("{}", "exit...");
}

// 初始化日志
fn init_logger() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::builder()
        .format(|buf, record| {
            let ts = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn | log::Level::Error => {
                    level_style.set_color(LColor::Red).set_bold(true)
                }
                _ => level_style.set_color(LColor::Blue).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {} {} {}] {}",
                ts,
                level_style.value(record.level()),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .last()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
