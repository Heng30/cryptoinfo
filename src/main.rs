use env_logger::fmt::Color as LColor;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::{QObjectPinned, QUrl};
use std::cell::RefCell;
use std::fs;
use std::io::Write;
use tokio;
use chrono::Local;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

mod res;

mod pricer;
use pricer::Model as pricer_model;

mod addition;
use addition::Addition as pricer_addition;

mod config;
use config::Config as conf;

mod todo;
use todo::Model as todo_model;

mod translator;
use translator::Translator as translation;

mod note;
use note::Note as private_note;

mod qbox;
use qbox::QBox;

mod download;

#[tokio::main]
async fn main() {
    init_logger();

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

    // 价值todo list
    let t_model = RefCell::new(todo_model::default());
    let todo_file = app_dirs.data_dir.join("todo.dat");
    t_model.borrow_mut().set_todo_path(todo_file.to_str().unwrap());
    t_model.borrow_mut().load();
    let t_model = unsafe { QObjectPinned::new(&t_model) };
    todo_model::init_from_engine(&mut engine, t_model);

    // 加载笔记
    let pnote = RefCell::new(private_note::default());
    let pnote_file = app_dirs.data_dir.join("note.dat");
    pnote
        .borrow_mut()
        .set_note_path(pnote_file.to_str().unwrap());
    pnote.borrow_mut().load_text();
    let pnote = unsafe { QObjectPinned::new(&pnote) };
    private_note::init_from_engine(&mut engine, pnote);

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
    pricer_model.borrow_mut().init_default(&config.borrow());
    pricer_model::init_from_engine(&mut engine, pricer_model);

    // 贪婪指数和时间（面板头信息)
    let pricer_addition = RefCell::new(pricer_addition::default());
    let pricer_addition = unsafe { QObjectPinned::new(&pricer_addition) };
    pricer_addition::init_from_engine(&mut engine, pricer_addition);

    // 定时更新
    download::update_price(QBox::new(pricer_model.borrow()));
    download::update_fear_greed(QBox::new(pricer_addition.borrow()));
    download::update_market(QBox::new(pricer_addition.borrow()));

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
                record.file().unwrap_or("None").split('/').last().unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
