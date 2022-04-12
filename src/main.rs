use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::{QObjectPinned, QUrl};

use env_logger as _;
use std::cell::RefCell;
use std::fs;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

mod res;

mod pricer;
use pricer::Model as pricer_model;

mod addtion;
use addtion::Addition as pricer_addtion;

mod config;
use config::Config as conf;

mod translator;
use translator::Translator as translation;

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    debug!("{}", "start...");

    res::resource_init();
    let app_dirs = init_app_dir();
    let mut engine = QmlEngine::new();

    // 加载配置文件
    let config = RefCell::new(conf::default());
    let config_file = app_dirs.config_dir.join("app.conf");
    config
        .borrow_mut()
        .set_config_path(config_file.to_str().unwrap());
    config.borrow_mut().load_config();

    let use_chinese = config.borrow().get_use_chinese();

    let config = unsafe { QObjectPinned::new(&config) };
    conf::init_from_engine(&mut engine, config);

    // 加载翻译文件
    let translator = RefCell::new(translation::default());
    translator.borrow_mut().set_use_chinese(use_chinese);
    let translator_file = app_dirs.config_dir.join("translation.dat");
    translator
        .borrow_mut()
        .set_translation_path(translator_file.to_str().unwrap());
    translator.borrow_mut().load_translation();

    let translator = unsafe { QObjectPinned::new(&translator) };
    translation::init_from_engine(&mut engine, translator);

    // 价格面板
    let pricer_model = RefCell::new(pricer_model::default());

    // 初始化价格相关的私有数据
    let private_data_file = app_dirs.data_dir.join("private.dat");
    pricer_model
        .borrow_mut()
        .set_private_data_path(private_data_file.to_str().unwrap());
    pricer_model.borrow_mut().load_private_data();

    let price_file = app_dirs.data_dir.join("price.dat");
    pricer_model
        .borrow_mut()
        .set_price_path(price_file.to_str().unwrap());
    pricer_model.borrow_mut().load_prices();

    let pricer_model = unsafe { QObjectPinned::new(&pricer_model) };
    pricer_model::init_from_engine(&mut engine, pricer_model);

    // 贪婪指数和时间（面板头信息)
    let pricer_addtion = RefCell::new(pricer_addtion::default());
    let pricer_addtion = unsafe { QObjectPinned::new(&pricer_addtion) };
    pricer_addtion::init_from_engine(&mut engine, pricer_addtion);

    engine.load_url(QUrl::from(QString::from("qrc:/res/qml/main.qml")));
    engine.exec();

    // 保证UI部分先被析构
    drop(engine);

    debug!("{}", "exit...");
}

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
