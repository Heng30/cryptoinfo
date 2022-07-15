use crate::config::Config;
use crate::database::LoginTable;
use crate::defi::{DefiChainModel, DefiChainNameModel, DefiChainTVLModel, DefiProtocolModel};
use crate::ghotkey::Ghotkey;
use crate::news::NewsModel;
use crate::price::{PriceAddition, PriceModel};
use crate::tool::{
    AddrBookModel, BookMarkModel, Encipher, FundBookModel, HandBookModel, Note, TodoModel,
};
use crate::translator::Translator;
use crate::utility::Utility;
use lazy_static;
use modeldata::{qcast_to, qcast_to_mut, QBox};
use pidlock::Pidlock;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

#[allow(unused_imports)]
use log::{debug, warn};

lazy_static! {
    static ref OBJMAP: Mutex<HashMap<NodeType, Node>> = Mutex::new(HashMap::new());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeType {
    UTILITY = 0,
    APPDIR = 1,
    CONFIG = 2,
    PIDLOCK = 3,
    #[allow(non_camel_case_types)]
    LOGIN_TABLE = 4,
    HOTKEY = 5,
    TRANSLATOR = 6,
    ENCIPHER = 7,
    #[allow(non_camel_case_types)]
    ADDRBOOK_MODEL = 8,
    #[allow(non_camel_case_types)]
    HANDBOOK_MODEL = 9,
    #[allow(non_camel_case_types)]
    TODO_MODEL = 10,
    NOTE = 11,
    #[allow(non_camel_case_types)]
    PRICE_MODEL = 12,
    #[allow(non_camel_case_types)]
    PRICE_ADDITION = 13,
    #[allow(non_camel_case_types)]
    DEFI_PROTOCOL_MODEL = 14,
    #[allow(non_camel_case_types)]
    DEFI_CHAIN_MODEL = 15,
    #[allow(non_camel_case_types)]
    DEFI_CHAIN_NAME_MODEL = 16,
    #[allow(non_camel_case_types)]
    DEFI_CHAIN_TVL_MODEL = 17,
    #[allow(non_camel_case_types)]
    BOOKMARK_MODEL = 20,
    #[allow(non_camel_case_types)]
    NEWS_MODEL = 21,
    #[allow(non_camel_case_types)]
    FUNDBOOK_MODEL = 23,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    pub ptr: usize,
}

impl Node {
    fn new<T>(ptr: &T) -> Node {
        return Node {
            ptr: ptr as *const T as usize,
        };
    }
}

pub fn qobj<'a, T>(ntype: NodeType) -> &'a T {
    let ptr = OBJMAP.lock().unwrap().get(&ntype).unwrap().ptr;
    return qcast_to::<T>(ptr);
}

pub fn qobj_mut<'a, T>(ntype: NodeType) -> &'a mut T {
    let ptr = OBJMAP.lock().unwrap().get(&ntype).unwrap().ptr;
    return qcast_to_mut::<T>(ptr);
}

// 创建目录
pub fn init_app_dir() -> Box<RefCell<AppDirs>> {
    let app_dirs = Box::new(RefCell::new(
        AppDirs::new(Some("cryptoinfo"), true).unwrap(),
    ));
    if let Err(_) = fs::create_dir_all(&app_dirs.borrow().data_dir) {
        warn!("create {:?} failed!!!", &app_dirs.borrow().data_dir);
    }

    if let Err(_) = fs::create_dir_all(app_dirs.borrow().data_dir.join("chain-tvl")) {
        warn!("create {:?} failed!!!", &app_dirs.borrow().data_dir);
    }

    if let Err(_) = fs::create_dir_all(app_dirs.borrow().data_dir.join("addrbook")) {
        warn!("create {:?} failed!!!", &app_dirs.borrow().data_dir);
    }

    if let Err(_) = fs::create_dir_all(&app_dirs.borrow().config_dir) {
        warn!("create {:?} failed!!!", &app_dirs.borrow().config_dir);
    }

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::APPDIR, Node::new(&*(app_dirs.borrow())));

    return app_dirs;
}

// 是否启动进程单实例
pub fn init_pidlock() -> Box<RefCell<Pidlock>> {
    let app_dirs = qobj::<AppDirs>(NodeType::APPDIR);
    let config = qobj_mut::<Config>(NodeType::CONFIG);
    let pidlock_path = app_dirs
        .data_dir
        .join("pid.lock")
        .to_str()
        .unwrap()
        .to_string();

    let pidlock = Box::new(RefCell::new(Pidlock::new(&pidlock_path)));
    if pidlock.borrow_mut().acquire().is_ok() {
        config.can_open_pidlock = true;
    } else {
        config.can_open_pidlock = false;
    }

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::PIDLOCK, Node::new(&*(pidlock.borrow())));
    return pidlock;
}

pub fn init_utility(engine: &mut QmlEngine) -> Box<RefCell<Utility>> {
    let utility = Box::new(RefCell::new(Utility::default()));
    Utility::init_from_engine(engine, unsafe { QObjectPinned::new(&utility) });
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::UTILITY, Node::new(&*(utility.borrow())));
    return utility;
}

// 加载配置文件
pub fn init_config(engine: &mut QmlEngine) -> Box<RefCell<Config>> {
    let config = Box::new(RefCell::new(Config::default()));
    Config::init_from_engine(engine, unsafe { QObjectPinned::new(&config) });
    config.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::CONFIG, Node::new(&*(config.borrow())));
    return config;
}

// 初始化登陆数据库
pub fn init_login_table(engine: &mut QmlEngine) -> Box<RefCell<LoginTable>> {
    let login_table = Box::new(RefCell::new(LoginTable::default()));
    LoginTable::init_from_engine(engine, unsafe { QObjectPinned::new(&login_table) });
    login_table.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::LOGIN_TABLE, Node::new(&*(login_table.borrow())));

    return login_table;
}

// 加载全局热键
pub fn init_hotkey(engine: &mut QmlEngine) -> Box<RefCell<Ghotkey>> {
    let hotkey = Box::new(RefCell::new(Ghotkey::default()));
    Ghotkey::init_from_engine(engine, unsafe { QObjectPinned::new(&hotkey) });
    Ghotkey::listen(QBox::new(&*hotkey.borrow()));
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::HOTKEY, Node::new(&*(hotkey.borrow())));

    return hotkey;
}

// 加载翻译文件
pub fn init_translator(engine: &mut QmlEngine) -> Box<RefCell<Translator>> {
    let translator = Box::new(RefCell::new(Translator::default()));
    Translator::init_from_engine(engine, unsafe { QObjectPinned::new(&translator) });
    translator.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::TRANSLATOR, Node::new(&*(translator.borrow())));
    return translator;
}

// toolbox 加解密工具
pub fn init_encipher(engine: &mut QmlEngine) -> Box<RefCell<Encipher>> {
    let enc = Box::new(RefCell::new(Encipher::default()));
    Encipher::init_from_engine(engine, unsafe { QObjectPinned::new(&enc) });
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ENCIPHER, Node::new(&*(enc.borrow())));

    return enc;
}

pub fn init_addrbook_model(engine: &mut QmlEngine) -> Box<RefCell<AddrBookModel>> {
    let addrbook_model = Box::new(RefCell::new(AddrBookModel::default()));
    AddrBookModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&addrbook_model) },
        "addrbook_model",
    );
    addrbook_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::ADDRBOOK_MODEL,
        Node::new(&*(addrbook_model.borrow())),
    );

    return addrbook_model;
}

pub fn init_handbook_model(engine: &mut QmlEngine) -> Box<RefCell<HandBookModel>> {
    let handbook_model = Box::new(RefCell::new(HandBookModel::default()));
    HandBookModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&handbook_model) },
        "handbook_model",
    );
    handbook_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::HANDBOOK_MODEL,
        Node::new(&*(handbook_model.borrow())),
    );
    return handbook_model;
}

pub fn init_fundbook_model(engine: &mut QmlEngine) -> Box<RefCell<FundBookModel>> {
    let model = Box::new(RefCell::new(FundBookModel::default()));
    FundBookModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "fundbook_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::FUNDBOOK_MODEL, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_bookmark_model(engine: &mut QmlEngine) -> Box<RefCell<BookMarkModel>> {
    let bookmark_model = Box::new(RefCell::new(BookMarkModel::default()));
    BookMarkModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&bookmark_model) },
        "bookmark_model",
    );
    bookmark_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::BOOKMARK_MODEL,
        Node::new(&*(bookmark_model.borrow())),
    );
    return bookmark_model;
}

// 价值todo list
pub fn init_todo_model(engine: &mut QmlEngine) -> Box<RefCell<TodoModel>> {
    let todo_model = Box::new(RefCell::new(TodoModel::default()));
    TodoModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&todo_model) },
        "todo_model",
    );
    todo_model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::TODO_MODEL, Node::new(&*(todo_model.borrow())));

    return todo_model;
}

// 加载笔记
pub fn init_note(engine: &mut QmlEngine) -> Box<RefCell<Note>> {
    let note = Box::new(RefCell::new(Note::default()));
    Note::init_from_engine(engine, unsafe { QObjectPinned::new(&note) });
    note.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::NOTE, Node::new(&*(note.borrow())));
    return note;
}

// 价格面板
pub fn init_price_model(engine: &mut QmlEngine) -> Box<RefCell<PriceModel>> {
    let price_model = Box::new(RefCell::new(PriceModel::default()));
    PriceModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&price_model) },
        "price_model",
    );
    price_model.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::PRICE_MODEL, Node::new(&*(price_model.borrow())));

    return price_model;
}

// 贪婪指数和时间（面板头信息)
pub fn init_price_addition(engine: &mut QmlEngine) -> Box<RefCell<PriceAddition>> {
    let price_addition = Box::new(RefCell::new(PriceAddition::default()));
    PriceAddition::init_from_engine(engine, unsafe { QObjectPinned::new(&price_addition) });

    price_addition.borrow_mut().init();
    OBJMAP.lock().unwrap().insert(
        NodeType::PRICE_ADDITION,
        Node::new(&*(price_addition.borrow())),
    );

    return price_addition;
}

pub fn init_defi_protocol_model(engine: &mut QmlEngine) -> Box<RefCell<DefiProtocolModel>> {
    let defi_protocol_model = Box::new(RefCell::new(DefiProtocolModel::default()));
    DefiProtocolModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&defi_protocol_model) },
        "defi_protocol_model",
    );
    defi_protocol_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::DEFI_PROTOCOL_MODEL,
        Node::new(&*(defi_protocol_model.borrow())),
    );

    return defi_protocol_model;
}

pub fn init_defi_chain_model(engine: &mut QmlEngine) -> Box<RefCell<DefiChainModel>> {
    let defi_chain_model = Box::new(RefCell::new(DefiChainModel::default()));
    DefiChainModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&defi_chain_model) },
        "defi_chain_model",
    );
    defi_chain_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::DEFI_CHAIN_MODEL,
        Node::new(&*(defi_chain_model.borrow())),
    );
    return defi_chain_model;
}

pub fn init_defi_chain_name_model(engine: &mut QmlEngine) -> Box<RefCell<DefiChainNameModel>> {
    let defi_chain_name_model = Box::new(RefCell::new(DefiChainNameModel::default()));
    DefiChainNameModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&defi_chain_name_model) },
        "defi_chain_name_model",
    );
    defi_chain_name_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::DEFI_CHAIN_NAME_MODEL,
        Node::new(&*(defi_chain_name_model.borrow())),
    );
    return defi_chain_name_model;
}

pub fn init_defi_chain_tvl_model(engine: &mut QmlEngine) -> Box<RefCell<DefiChainTVLModel>> {
    let defi_chain_tvl_model = Box::new(RefCell::new(DefiChainTVLModel::default()));
    DefiChainTVLModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&defi_chain_tvl_model) },
        "defi_chain_tvl_model",
    );
    defi_chain_tvl_model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::DEFI_CHAIN_TVL_MODEL,
        Node::new(&*(defi_chain_tvl_model.borrow())),
    );
    return defi_chain_tvl_model;
}

pub fn init_news_model(engine: &mut QmlEngine) -> Box<RefCell<NewsModel>> {
    let model = Box::new(RefCell::new(NewsModel::default()));
    NewsModel::init_from_engine(engine, unsafe { QObjectPinned::new(&model) }, "news_model");
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::NEWS_MODEL, Node::new(&*(model.borrow())));
    return model;
}
