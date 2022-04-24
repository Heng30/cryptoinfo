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
mod qbox;
mod res;
mod tool;
mod translator;
mod utility;

use config::Config;
use defi::{DefiChainModel, DefiDownload, DefiProtocolModel, DefiTotalTVLModel};
use panel::{Note, TodoModel};
use price::{PriceAddition, PriceDownload, PriceModel};
use qbox::QBox;
use tool::Encipher;
use translator::Translator;

#[tokio::main]
async fn main() {
    init_logger();

    debug!("{}", "start...");

    res::resource_init();
    let app_dirs = init_app_dir();
    let mut engine = QmlEngine::new();

    // 加载公用函数类
    let utili = RefCell::new(utility::Utility::default());
    let utili = unsafe { QObjectPinned::new(&utili) };
    utility::Utility::init_from_engine(&mut engine, utili);

    // 加载配置文件
    let config = RefCell::new(Config::default());
    let config_file = app_dirs.config_dir.join("app.conf");
    config.borrow_mut().set_path(config_file.to_str().unwrap());
    config.borrow_mut().load();

    let use_chinese = config.borrow().get_use_chinese();
    let config = unsafe { QObjectPinned::new(&config) };
    Config::init_from_engine(&mut engine, config);

    // 加载翻译文件
    let translator = RefCell::new(Translator::default());
    translator.borrow_mut().set_use_chinese(use_chinese);
    let translator_file = app_dirs.config_dir.join("translation.dat");
    translator
        .borrow_mut()
        .set_path(translator_file.to_str().unwrap());
    translator.borrow_mut().load();

    let translator = unsafe { QObjectPinned::new(&translator) };
    Translator::init_from_engine(&mut engine, translator);

    // toolbox 加解密工具
    let enc = RefCell::new(Encipher::default());
    let enc = unsafe { QObjectPinned::new(&enc) };
    Encipher::init_from_engine(&mut engine, enc);

    // 价值todo list
    let t_model = RefCell::new(TodoModel::default());
    let todo_file = app_dirs.data_dir.join("todo.dat");
    t_model.borrow_mut().set_path(todo_file.to_str().unwrap());
    t_model.borrow_mut().load();
    let t_model = unsafe { QObjectPinned::new(&t_model) };
    TodoModel::init_from_engine(&mut engine, t_model);

    // 加载笔记
    let pnote = RefCell::new(Note::default());
    let pnote_file = app_dirs.data_dir.join("note.dat");
    pnote.borrow_mut().set_path(pnote_file.to_str().unwrap());
    pnote.borrow_mut().load();
    let pnote = unsafe { QObjectPinned::new(&pnote) };
    Note::init_from_engine(&mut engine, pnote);

    // 价格面板
    let price_model = RefCell::new(PriceModel::default());
    price_model.borrow_mut().init_default(&config.borrow());

    // 初始化价格相关的私有数据
    let private_file = app_dirs.data_dir.join("private.json");
    price_model
        .borrow_mut()
        .set_private_path(private_file.to_str().unwrap());
    price_model.borrow_mut().load_private();

    let price_file = app_dirs.data_dir.join("price.json");
    price_model
        .borrow_mut()
        .set_price_path(price_file.to_str().unwrap());
    price_model.borrow_mut().load();
    let price_model = unsafe { QObjectPinned::new(&price_model) };
    PriceModel::init_from_engine(&mut engine, price_model);

    // 贪婪指数和时间（面板头信息)
    let price_addition = RefCell::new(PriceAddition::default());
    let price_addition = unsafe { QObjectPinned::new(&price_addition) };
    PriceAddition::init_from_engine(&mut engine, price_addition);

    // defi
    let defi_protocol_model = RefCell::new(DefiProtocolModel::default());
    defi_protocol_model
        .borrow_mut()
        .init_default(&config.borrow());
    let defi_protocol_file = app_dirs.data_dir.join("defi-protocols.json");
    defi_protocol_model
        .borrow_mut()
        .set_path(defi_protocol_file.to_str().unwrap());
    defi_protocol_model.borrow_mut().load();
    let defi_protocol_model = unsafe { QObjectPinned::new(&defi_protocol_model) };
    DefiProtocolModel::init_from_engine(&mut engine, defi_protocol_model);

    let defi_chain_model = RefCell::new(DefiChainModel::default());
    defi_chain_model.borrow_mut().init_default(&config.borrow());
    let defi_chain_file = app_dirs.data_dir.join("defi-chains.json");
    defi_chain_model
        .borrow_mut()
        .set_path(defi_chain_file.to_str().unwrap());
    defi_chain_model.borrow_mut().load();
    let defi_chain_model = unsafe { QObjectPinned::new(&defi_chain_model) };
    DefiChainModel::init_from_engine(&mut engine, defi_chain_model);

    let defi_total_tvl_model = RefCell::new(DefiTotalTVLModel::default());
    defi_total_tvl_model.borrow_mut().init_default(&config.borrow());
    let defi_total_tvl_file = app_dirs.data_dir.join("defi-total-tvl.json");
    defi_total_tvl_model.borrow_mut().set_path(defi_total_tvl_file.to_str().unwrap());
    defi_total_tvl_model.borrow_mut().load();
    let defi_total_tvl_model = unsafe { QObjectPinned::new(&defi_total_tvl_model) };
    DefiTotalTVLModel::init_from_engine(&mut engine, defi_total_tvl_model);

    // 定时更新
    let price_download = PriceDownload::default();
    price_download.update_price(QBox::new(price_model.borrow()));
    price_download.update_fear_greed(QBox::new(price_addition.borrow()));
    price_download.update_market(QBox::new(price_addition.borrow()));

    let defi_download = DefiDownload::default();
    defi_download.update_defi_protocol(QBox::new(defi_protocol_model.borrow()));
    defi_download.update_defi_chain(QBox::new(defi_chain_model.borrow()));
    defi_download.update_defi_total_tvl(QBox::new(defi_total_tvl_model.borrow()));


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
