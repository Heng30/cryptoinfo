#[macro_use]
extern crate lazy_static;

use chrono::Local;
use env_logger::fmt::Color as LColor;
use log::debug;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;
use std::io::Write;

mod account;
mod chain;
mod chart;
mod config;
mod database;
mod ghotkey;
mod httpclient;
mod intel;
mod notify;
mod price;
mod qobjmgr;
mod res;
mod stablecoin;
mod tool;
mod translator;
mod utility;
mod version;

#[tokio::main]
async fn main() {
    init_logger();

    debug!("{}", "start...");

    res::resource_init();
    let mut engine = QmlEngine::new();

    let _app_dir = qobjmgr::init_app_dir();
    let _utility = qobjmgr::init_utility(&mut engine);
    let _debug_log = qobjmgr::init_debug_log(&mut engine);
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
    let _note_model = qobjmgr::init_note_model(&mut engine);
    let _price_model = qobjmgr::init_price_model(&mut engine);
    let _price_addition = qobjmgr::init_price_addition(&mut engine);
    let _defi_protocol_model = qobjmgr::init_chain_protocol_model(&mut engine);
    let _defi_chain_model = qobjmgr::init_chain_tvl_model(&mut engine);
    let _defi_chain_name_model = qobjmgr::init_chain_name_model(&mut engine);
    let _defi_chain_tvl_model = qobjmgr::init_chart_chain_tvl_model(&mut engine);
    let _macro_event = qobjmgr::init_macro_event_model(&mut engine);
    let _macro_news = qobjmgr::init_macro_news_model(&mut engine);
    let _stable_coin_mcap_model = qobjmgr::init_stable_coin_mcap_model(&mut engine);
    let _stable_coin_chain_model = qobjmgr::init_stable_coin_chain_model(&mut engine);
    let _chain_yield_model = qobjmgr::init_chain_yield_model(&mut engine);
    let _okex_account = qobjmgr::init_okex_account(&mut engine);
    let _okex_sub_sta_model = qobjmgr::init_okex_subscribe_status_model(&mut engine);
    let _okex_acc_chan_model = qobjmgr::init_okex_account_channel_model(&mut engine);
    let _okex_pos_chan_model = qobjmgr::init_okex_position_channel_model(&mut engine);
    let _okex_greek_chan_model = qobjmgr::init_okex_greek_channel_model(&mut engine);
    let _okex_main_acc_rest_model = qobjmgr::init_okex_main_account_rest_model(&mut engine);
    let _okex_deposit_rest_model = qobjmgr::init_okex_deposit_rest_model(&mut engine);
    let _okex_withdrawal_rest_model = qobjmgr::init_okex_withdrawal_rest_model(&mut engine);
    let _okex_bill_rest_model = qobjmgr::init_okex_bill_rest_model(&mut engine);
    let _contract_stats_model = qobjmgr::init_contract_stats_model(&mut engine);
    let _notify_model = qobjmgr::init_notify_model(&mut engine);
    let _crypto_fee_model = qobjmgr::init_crypto_fee_model(&mut engine);

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
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn | log::Level::Error => {
                    level_style.set_color(LColor::Red).set_bold(true)
                }
                _ => level_style.set_color(LColor::Blue).set_bold(true),
            };

            tool::DebugLog::send(format!(
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
            ));

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
