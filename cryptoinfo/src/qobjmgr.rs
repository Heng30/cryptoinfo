use crate::account::{
    OkexAccChanModel, OkexAccount, OkexBillRestModel, OkexDepositRestModel, OkexGreekChanModel,
    OkexMainAccRestModel, OkexPosChanModel, OkexSubStaModel, OkexWithdrawalRestModel,
};
use crate::address::AddressEthModel;
use crate::chain::{
    ChainEthTokenModel, ChainNameModel, ChainProtocolModel, ChainTvlModel, ChainYieldModel,
};
use crate::chart::ChartChainTVLModel;
use crate::config::Config;
use crate::database::LoginTable;
use crate::exchange::ExchangeBtcModel;
use crate::ghotkey::Ghotkey;
use crate::monitor::{MonitorBtcModel, MonitorEthModel};
use crate::news::NewsModel;
use crate::price::{PriceAddition, PriceModel};
use crate::stablecoin::{StableCoinChainModel, StableCoinMcapModel};
use crate::tool::{
    AddrBookModel, BookMarkModel, DebugLog, Encipher, FundBookModel, HandBookModel, Note, TodoModel,
};
use crate::translator::Translator;
use crate::utility::Utility;
use lazy_static;
#[allow(unused_imports)]
use log::{debug, warn};
use modeldata::{qcast_to, qcast_to_mut, QBox};
use pidlock::Pidlock;
use platform_dirs::AppDirs;
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    static ref OBJMAP: Mutex<HashMap<NodeType, Node>> = Mutex::new(HashMap::new());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeType {
    Utility = 0,
    AppDir = 1,
    Config = 2,
    Pidlock = 3,
    LoginTable = 4,
    Hotkey = 5,
    Translator = 6,
    Encipher = 7,
    AddrBookModel = 8,
    HandBookModel = 9,
    TodoModel = 10,
    Note = 11,
    PriceModel = 12,
    PriceAddition = 13,
    ChainProtocolModel = 14,
    ChainTvlModel = 15,
    ChainNameModel = 16,
    ChartChainTvlModel = 17,
    ExchangeBtcModel = 18,
    MonitorBtcModel = 19,
    BookMarkModel = 20,
    NewsModel = 21,
    StableCoinMcapModel = 22,
    FundBookModel = 23,
    StableCoinChainModel = 24,
    ChainYieldModel = 25,
    MonitorEthModel = 26,
    AddressEthModel = 27,
    ChainEthTokenModel = 28,
    OkexAccount = 29,
    OkexSubStaModel = 30,
    OkexAccChanModel = 31,
    OkexPosChanModel = 32,
    OkexGreekChanModel = 33,
    OkexMainAccRestModel = 34,
    OkexDepositRestModel = 35,
    OkexWithdrawalRestModel = 36,
    OkexBillRestModel = 37,
    DebugLog = 38,
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

pub fn contain_obj(ntype: NodeType) -> bool {
    return OBJMAP.lock().unwrap().contains_key(&ntype);
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
        .insert(NodeType::AppDir, Node::new(&*(app_dirs.borrow())));

    return app_dirs;
}

// 是否启动进程单实例
pub fn init_pidlock() -> Box<RefCell<Pidlock>> {
    let app_dirs = qobj::<AppDirs>(NodeType::AppDir);
    let config = qobj_mut::<Config>(NodeType::Config);
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
        .insert(NodeType::Pidlock, Node::new(&*(pidlock.borrow())));
    return pidlock;
}

pub fn init_utility(engine: &mut QmlEngine) -> Box<RefCell<Utility>> {
    let utility = Box::new(RefCell::new(Utility::default()));
    Utility::init_from_engine(engine, unsafe { QObjectPinned::new(&utility) });
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::Utility, Node::new(&*(utility.borrow())));
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
        .insert(NodeType::Config, Node::new(&*(config.borrow())));
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
        .insert(NodeType::LoginTable, Node::new(&*(login_table.borrow())));

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
        .insert(NodeType::Hotkey, Node::new(&*(hotkey.borrow())));

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
        .insert(NodeType::Translator, Node::new(&*(translator.borrow())));
    return translator;
}

// toolbox 加解密工具
pub fn init_encipher(engine: &mut QmlEngine) -> Box<RefCell<Encipher>> {
    let enc = Box::new(RefCell::new(Encipher::default()));
    Encipher::init_from_engine(engine, unsafe { QObjectPinned::new(&enc) });
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::Encipher, Node::new(&*(enc.borrow())));

    return enc;
}

pub fn init_addrbook_model(engine: &mut QmlEngine) -> Box<RefCell<AddrBookModel>> {
    let model = Box::new(RefCell::new(AddrBookModel::default()));
    AddrBookModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "addrbook_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::AddrBookModel, Node::new(&*(model.borrow())));

    return model;
}

pub fn init_handbook_model(engine: &mut QmlEngine) -> Box<RefCell<HandBookModel>> {
    let model = Box::new(RefCell::new(HandBookModel::default()));
    HandBookModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "handbook_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::HandBookModel, Node::new(&*(model.borrow())));
    return model;
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
        .insert(NodeType::FundBookModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_bookmark_model(engine: &mut QmlEngine) -> Box<RefCell<BookMarkModel>> {
    let model = Box::new(RefCell::new(BookMarkModel::default()));
    BookMarkModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "bookmark_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::BookMarkModel, Node::new(&*(model.borrow())));
    return model;
}

// 价值todo list
pub fn init_todo_model(engine: &mut QmlEngine) -> Box<RefCell<TodoModel>> {
    let model = Box::new(RefCell::new(TodoModel::default()));
    TodoModel::init_from_engine(engine, unsafe { QObjectPinned::new(&model) }, "todo_model");
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::TodoModel, Node::new(&*(model.borrow())));

    return model;
}

pub fn init_debug_log(engine: &mut QmlEngine) -> Box<RefCell<DebugLog>> {
    let obj = Box::new(RefCell::new(DebugLog::default()));
    DebugLog::init_from_engine(engine, unsafe { QObjectPinned::new(&obj) });
    obj.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::DebugLog, Node::new(&*(obj.borrow())));

    return obj;
}

// 加载笔记
pub fn init_note(engine: &mut QmlEngine) -> Box<RefCell<Note>> {
    let note = Box::new(RefCell::new(Note::default()));
    Note::init_from_engine(engine, unsafe { QObjectPinned::new(&note) });
    note.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::Note, Node::new(&*(note.borrow())));

    return note;
}

// 价格面板
pub fn init_price_model(engine: &mut QmlEngine) -> Box<RefCell<PriceModel>> {
    let model = Box::new(RefCell::new(PriceModel::default()));
    PriceModel::init_from_engine(engine, unsafe { QObjectPinned::new(&model) }, "price_model");
    model.borrow_mut().init();
    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::PriceModel, Node::new(&*(model.borrow())));

    return model;
}

// 贪婪指数和时间（面板头信息)
pub fn init_price_addition(engine: &mut QmlEngine) -> Box<RefCell<PriceAddition>> {
    let price_addition = Box::new(RefCell::new(PriceAddition::default()));
    PriceAddition::init_from_engine(engine, unsafe { QObjectPinned::new(&price_addition) });

    price_addition.borrow_mut().init();
    OBJMAP.lock().unwrap().insert(
        NodeType::PriceAddition,
        Node::new(&*(price_addition.borrow())),
    );

    return price_addition;
}

pub fn init_chain_protocol_model(engine: &mut QmlEngine) -> Box<RefCell<ChainProtocolModel>> {
    let model = Box::new(RefCell::new(ChainProtocolModel::default()));
    ChainProtocolModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chain_protocol_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChainProtocolModel, Node::new(&*(model.borrow())));

    return model;
}

pub fn init_chain_tvl_model(engine: &mut QmlEngine) -> Box<RefCell<ChainTvlModel>> {
    let model = Box::new(RefCell::new(ChainTvlModel::default()));
    ChainTvlModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chain_tvl_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChainTvlModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_chain_name_model(engine: &mut QmlEngine) -> Box<RefCell<ChainNameModel>> {
    let model = Box::new(RefCell::new(ChainNameModel::default()));
    ChainNameModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chain_name_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChainNameModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_chart_chain_tvl_model(engine: &mut QmlEngine) -> Box<RefCell<ChartChainTVLModel>> {
    let model = Box::new(RefCell::new(ChartChainTVLModel::default()));
    ChartChainTVLModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chart_chain_tvl_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChartChainTvlModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_news_model(engine: &mut QmlEngine) -> Box<RefCell<NewsModel>> {
    let model = Box::new(RefCell::new(NewsModel::default()));
    NewsModel::init_from_engine(engine, unsafe { QObjectPinned::new(&model) }, "news_model");
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::NewsModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_exchange_btc_model(engine: &mut QmlEngine) -> Box<RefCell<ExchangeBtcModel>> {
    let model = Box::new(RefCell::new(ExchangeBtcModel::default()));
    ExchangeBtcModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "exchange_btc_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ExchangeBtcModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_monitor_btc_model(engine: &mut QmlEngine) -> Box<RefCell<MonitorBtcModel>> {
    let model = Box::new(RefCell::new(MonitorBtcModel::default()));
    MonitorBtcModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "monitor_btc_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::MonitorBtcModel, Node::new(&*(model.borrow())));

    return model;
}

pub fn init_monitor_eth_model(engine: &mut QmlEngine) -> Box<RefCell<MonitorEthModel>> {
    let model = Box::new(RefCell::new(MonitorEthModel::default()));
    MonitorEthModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "monitor_eth_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::MonitorEthModel, Node::new(&*(model.borrow())));

    return model;
}

pub fn init_stable_coin_mcap_model(engine: &mut QmlEngine) -> Box<RefCell<StableCoinMcapModel>> {
    let model = Box::new(RefCell::new(StableCoinMcapModel::default()));
    StableCoinMcapModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "stable_coin_mcap_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::StableCoinMcapModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_stable_coin_chain_model(engine: &mut QmlEngine) -> Box<RefCell<StableCoinChainModel>> {
    let model = Box::new(RefCell::new(StableCoinChainModel::default()));
    StableCoinChainModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "stable_coin_chain_model",
    );
    model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::StableCoinChainModel,
        Node::new(&*(model.borrow())),
    );
    return model;
}

pub fn init_chain_yield_model(engine: &mut QmlEngine) -> Box<RefCell<ChainYieldModel>> {
    let model = Box::new(RefCell::new(ChainYieldModel::default()));
    ChainYieldModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chain_yield_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChainYieldModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_address_eth_model(engine: &mut QmlEngine) -> Box<RefCell<AddressEthModel>> {
    let model = Box::new(RefCell::new(AddressEthModel::default()));
    AddressEthModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "address_eth_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::AddressEthModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_chain_eth_token_model(engine: &mut QmlEngine) -> Box<RefCell<ChainEthTokenModel>> {
    let model = Box::new(RefCell::new(ChainEthTokenModel::default()));
    ChainEthTokenModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "chain_eth_token_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::ChainEthTokenModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_okex_account(engine: &mut QmlEngine) -> Box<RefCell<OkexAccount>> {
    let account = Box::new(RefCell::new(OkexAccount::default()));
    OkexAccount::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&account) },
        "okex_account",
    );
    account.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexAccount, Node::new(&*(account.borrow())));
    return account;
}

pub fn init_okex_subscribe_status_model(engine: &mut QmlEngine) -> Box<RefCell<OkexSubStaModel>> {
    let model = Box::new(RefCell::new(OkexSubStaModel::default()));
    OkexSubStaModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_subscribe_status_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexSubStaModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_okex_account_channel_model(engine: &mut QmlEngine) -> Box<RefCell<OkexAccChanModel>> {
    let model = Box::new(RefCell::new(OkexAccChanModel::default()));
    OkexAccChanModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_account_channel_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexAccChanModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_okex_position_channel_model(engine: &mut QmlEngine) -> Box<RefCell<OkexPosChanModel>> {
    let model = Box::new(RefCell::new(OkexPosChanModel::default()));
    OkexPosChanModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_position_channel_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexPosChanModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_okex_greek_channel_model(engine: &mut QmlEngine) -> Box<RefCell<OkexGreekChanModel>> {
    let model = Box::new(RefCell::new(OkexGreekChanModel::default()));
    OkexGreekChanModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_greek_channel_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexGreekChanModel, Node::new(&*(model.borrow())));
    return model;
}

pub fn init_okex_main_account_rest_model(
    engine: &mut QmlEngine,
) -> Box<RefCell<OkexMainAccRestModel>> {
    let model = Box::new(RefCell::new(OkexMainAccRestModel::default()));
    OkexMainAccRestModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_main_account_rest_model",
    );
    model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::OkexMainAccRestModel,
        Node::new(&*(model.borrow())),
    );
    return model;
}

pub fn init_okex_deposit_rest_model(engine: &mut QmlEngine) -> Box<RefCell<OkexDepositRestModel>> {
    let model = Box::new(RefCell::new(OkexDepositRestModel::default()));
    OkexDepositRestModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_deposit_rest_model",
    );
    model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::OkexDepositRestModel,
        Node::new(&*(model.borrow())),
    );
    return model;
}

pub fn init_okex_withdrawal_rest_model(
    engine: &mut QmlEngine,
) -> Box<RefCell<OkexWithdrawalRestModel>> {
    let model = Box::new(RefCell::new(OkexWithdrawalRestModel::default()));
    OkexWithdrawalRestModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_withdrawal_rest_model",
    );
    model.borrow_mut().init();

    OBJMAP.lock().unwrap().insert(
        NodeType::OkexWithdrawalRestModel,
        Node::new(&*(model.borrow())),
    );
    return model;
}

pub fn init_okex_bill_rest_model(engine: &mut QmlEngine) -> Box<RefCell<OkexBillRestModel>> {
    let model = Box::new(RefCell::new(OkexBillRestModel::default()));
    OkexBillRestModel::init_from_engine(
        engine,
        unsafe { QObjectPinned::new(&model) },
        "okex_bill_rest_model",
    );
    model.borrow_mut().init();

    OBJMAP
        .lock()
        .unwrap()
        .insert(NodeType::OkexBillRestModel, Node::new(&*(model.borrow())));
    return model;
}
