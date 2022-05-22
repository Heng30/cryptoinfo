use chrono::Local;
use env_logger::fmt::Color as LColor;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::{QObjectPinned, QUrl};
use std::cell::RefCell;
use std::fs;
use std::io::Write;
use tokio;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

mod config;
mod defi;
mod panel;
mod price;
mod res;
mod tool;
mod translator;
mod utility;
mod ghotkey;

use config::Config;
use defi::{DefiChainModel, DefiDownload, DefiProtocolModel, DefiChainNameModel, DefiChainTVLModel};
use modeldata::QBox;
use panel::{Note, TodoModel};
use price::{PriceAddition, PriceDownload, PriceModel};
use tool::{Encipher, AddrBookModel, HandBookModel};
use translator::Translator;

#[tokio::main]
async fn main() {
    init_logger();

    debug!("{}", "start...");

    res::resource_init();
    let app_dirs = init_app_dir();
    let mut engine = QmlEngine::new();

    // 加载公用函数类
    let utility = RefCell::new(utility::Utility::default());
    let utility = unsafe { QObjectPinned::new(&utility) };
    utility::Utility::init_from_engine(&mut engine, utility);

    // 加载配置文件
    let config = RefCell::new(Config::default());
    let config = unsafe { QObjectPinned::new(&config) };
    Config::init_from_engine(&mut engine, config);
    config.borrow_mut().init(&app_dirs);

    // 加载全局热键
    let hotkey = RefCell::new(ghotkey::Ghotkey::default());
    let hotkey = unsafe { QObjectPinned::new(&hotkey) };
    ghotkey::Ghotkey::init_from_engine(&mut engine, hotkey);
    ghotkey::Ghotkey::listen(QBox::new(hotkey.borrow()));

    // 加载翻译文件
    let translator = RefCell::new(Translator::default());
    let translator = unsafe { QObjectPinned::new(&translator) };
    Translator::init_from_engine(&mut engine, translator);
    translator.borrow_mut().init(config.borrow(), &app_dirs);

    // toolbox 加解密工具
    let enc = RefCell::new(Encipher::default());
    let enc = unsafe { QObjectPinned::new(&enc) };
    Encipher::init_from_engine(&mut engine, enc);

    let addrbook_model = RefCell::new(AddrBookModel::default());
    let addrbook_model = unsafe { QObjectPinned::new(&addrbook_model) };
    AddrBookModel::init_from_engine(&mut engine, addrbook_model, "addrbook_model");
    addrbook_model.borrow_mut().init(&app_dirs);

    let handbook_model = RefCell::new(HandBookModel::default());
    let handbook_model = unsafe { QObjectPinned::new(&handbook_model) };
    HandBookModel::init_from_engine(&mut engine, handbook_model, "handbook_model");
    handbook_model.borrow_mut().init(&app_dirs);

    // 价值todo list
    let todo_model = RefCell::new(TodoModel::default());
    let todo_model = unsafe { QObjectPinned::new(&todo_model) };
    TodoModel::init_from_engine(&mut engine, todo_model, "todo_model");
    todo_model.borrow_mut().init(&app_dirs);

    // 加载笔记
    let pnote = RefCell::new(Note::default());
    let pnote = unsafe { QObjectPinned::new(&pnote) };
    Note::init_from_engine(&mut engine, pnote);
    pnote.borrow_mut().init(&app_dirs);

    // 价格面板
    let price_model = RefCell::new(PriceModel::default());
    let price_model = unsafe { QObjectPinned::new(&price_model) };
    PriceModel::init_from_engine(&mut engine, price_model, "price_model");
    price_model.borrow_mut().init(&config.borrow(), &app_dirs);

    // 贪婪指数和时间（面板头信息)
    let price_addition = RefCell::new(PriceAddition::default());
    let price_addition = unsafe { QObjectPinned::new(&price_addition) };
    PriceAddition::init_from_engine(&mut engine, price_addition);

    let defi_protocol_model = RefCell::new(DefiProtocolModel::default());
    let defi_protocol_model = unsafe { QObjectPinned::new(&defi_protocol_model) };
    DefiProtocolModel::init_from_engine(&mut engine, defi_protocol_model, "defi_protocol_model");
    defi_protocol_model
        .borrow_mut()
        .init(config.borrow(), &app_dirs);

    let defi_chain_model = RefCell::new(DefiChainModel::default());
    let defi_chain_model = unsafe { QObjectPinned::new(&defi_chain_model) };
    DefiChainModel::init_from_engine(&mut engine, defi_chain_model, "defi_chain_model");
    defi_chain_model
        .borrow_mut()
        .init(config.borrow(), &app_dirs);

    let defi_chain_name_model = RefCell::new(DefiChainNameModel::default());
    let defi_chain_name_model = unsafe { QObjectPinned::new(&defi_chain_name_model) };
    DefiChainNameModel::init_from_engine(&mut engine, defi_chain_name_model, "defi_chain_name_model");
    defi_chain_name_model.borrow_mut().init(&app_dirs);

    let defi_chain_tvl_model = RefCell::new(DefiChainTVLModel::default());
    let defi_chain_tvl_model = unsafe { QObjectPinned::new(&defi_chain_tvl_model) };
    DefiChainTVLModel::init_from_engine(&mut engine, defi_chain_tvl_model, "defi_chain_tvl_model");
    defi_chain_tvl_model.borrow_mut().init(&app_dirs);

    // 定时更新
    let price_download = PriceDownload::default();
    price_download.update_price(QBox::new(price_model.borrow()));
    price_download.update_market(QBox::new(price_addition.borrow()));
    price_download.update_fear_greed(QBox::new(price_addition.borrow()));
    price_download.update_eth_gas(QBox::new(price_addition.borrow()));
    price_download.update_btc_stats(QBox::new(price_addition.borrow()));

    let defi_download = DefiDownload::default();
    defi_download.update_defi_chain(QBox::new(defi_chain_model.borrow()));
    defi_download.update_defi_protocol(QBox::new(defi_protocol_model.borrow()));
    defi_download.update_defi_chain_tvl(QBox::new(defi_chain_tvl_model.borrow()));

    engine.load_url(QUrl::from(QString::from("qrc:/res/qml/main.qml")));
    engine.exec();

    // 保证UI部分先被析构
    drop(engine);

    debug!("{}", "exit...");
}

// 创建目录
fn init_app_dir() -> AppDirs {
    let app_dirs = AppDirs::new(Some("cryptoinfo"), true).unwrap();
    if let Err(_) = fs::create_dir_all(&app_dirs.data_dir) {
        warn!("create {:?} failed!!!", &app_dirs.data_dir);
    }

    if let Err(_) = fs::create_dir_all(app_dirs.data_dir.join("chain-tvl")) {
        warn!("create {:?} failed!!!", &app_dirs.data_dir);
    }

    if let Err(_) = fs::create_dir_all(app_dirs.data_dir.join("addrbook")) {
        warn!("create {:?} failed!!!", &app_dirs.data_dir);
    }

    if let Err(_) = fs::create_dir_all(&app_dirs.config_dir) {
        warn!("create {:?} failed!!!", &app_dirs.config_dir);
    }
    return app_dirs;
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
