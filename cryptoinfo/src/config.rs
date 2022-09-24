use crate::qobjmgr::{qobj, NodeType as QNodeType};
use ::log::{debug, warn};
use cstr::cstr;
use platform_dirs::AppDirs;
use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
pub enum PanelType {
    Unknown = 0,
    Price = 1,
    Setting = 2,
    Intel = 3,
    Exchange = 4,
    ToolBox = 5,
    Address = 6,
    Chain = 7,
    Chart = 8,
    Monitor = 9,
    StableCoin = 10,
    Account = 11,
    DebugLog = 12,
    NFT = 13,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawConfig {
    font_pixel_size_normal: u32,
    is_dark_theme: bool,
    is_window_mode: bool,
    use_chinese: bool,
    show_splash: bool,
    use_login_sound: bool,
    enable_web_server: bool,
    web_server_address: String,
    web_server_port: u32,
    enable_login_password: bool,
    single_ins: bool,
    splash_interval: u32,
    window_opacity: f32,
    window_width: f32,
    window_height: f32,
    price_refresh_interval: u32, // 数据刷新时间间隔
    price_item_count: u32,       // 价格条目数量
    browser: String,
    owlracle_api_key: String,
    okex_api_key: String,
    okex_passphrase: String,
    okex_secret_key: String,
    okex_websocket_is_start_enable: bool,

    // 避免数据量大的任务在不关注时刷新, 造成UI卡顿, websocket不受影响,
    unrefresh_when_not_focus: bool,
}

impl Default for RawConfig {
    fn default() -> Self {
        return Self {
            font_pixel_size_normal: 16,
            is_dark_theme: false,
            is_window_mode: true,
            use_chinese: true,
            show_splash: false,
            use_login_sound: false,
            enable_web_server: false,
            web_server_address: "127.0.0.1".to_string(),
            web_server_port: 8000,
            enable_login_password: false,
            single_ins: false,
            splash_interval: 3000,
            window_opacity: 1.0,
            window_width: 1300f32,
            window_height: 1000f32,
            price_refresh_interval: 30,
            price_item_count: 100,
            browser: "brave".to_string(),
            owlracle_api_key: "".to_string(),
            okex_api_key: "".to_string(),
            okex_passphrase: "".to_string(),
            okex_secret_key: "".to_string(),
            okex_websocket_is_start_enable: false,
            unrefresh_when_not_focus: false,
        };
    }
}

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),
    path: String,
    config_dir: qt_property!(QString),
    data_dir: qt_property!(QString),
    working_dir: qt_property!(QString),

    pub can_open_pidlock: qt_property!(bool),

    // UI
    font_pixel_size_normal: qt_property!(u32; NOTIFY font_pixel_size_normal_changed),
    font_pixel_size_normal_changed: qt_signal!(),

    is_dark_theme: qt_property!(bool; NOTIFY is_dark_theme_changed),
    is_dark_theme_changed: qt_signal!(),

    is_window_mode: qt_property!(bool; NOTIFY is_window_mode_changed),
    is_window_mode_changed: qt_signal!(),

    pub use_chinese: qt_property!(bool; NOTIFY use_chinese_changed),
    use_chinese_changed: qt_signal!(),

    splash_interval: qt_property!(u32; NOTIFY splash_interval_changed),
    splash_interval_changed: qt_signal!(),
    show_splash: qt_property!(bool; NOTIFY show_splash_changed),
    show_splash_changed: qt_signal!(),

    use_login_sound: qt_property!(bool; NOTIFY use_login_sound_changed),
    use_login_sound_changed: qt_signal!(),

    pub enable_web_server: qt_property!(bool; NOTIFY enable_web_server_changed),
    enable_web_server_changed: qt_signal!(),

    pub web_server_address: qt_property!(QString; NOTIFY web_server_address_changed),
    web_server_address_changed: qt_signal!(),
    pub web_server_port: qt_property!(u32; NOTIFY web_server_port_changed),
    web_server_port_changed: qt_signal!(),

    pub owlracle_api_key: qt_property!(QString; NOTIFY owlracle_api_key_changed),
    owlracle_api_key_changed: qt_signal!(),

    pub okex_api_key: qt_property!(QString; NOTIFY okex_api_key_changed),
    okex_api_key_changed: qt_signal!(),
    pub okex_passphrase: qt_property!(QString; NOTIFY okex_passphrase_changed),
    okex_passphrase_changed: qt_signal!(),
    pub okex_secret_key: qt_property!(QString; NOTIFY okex_secret_key_changed),
    okex_secret_key_changed: qt_signal!(),

    pub okex_websocket_is_start_enable: qt_property!(bool; NOTIFY okex_websocket_is_start_enable_changed),
    okex_websocket_is_start_enable_changed: qt_signal!(),

    pub unrefresh_when_not_focus: qt_property!(bool; NOTIFY unrefresh_when_not_focus_changed),
    unrefresh_when_not_focus_changed: qt_signal!(),

    enable_login_password: qt_property!(bool; NOTIFY enable_login_password_changed),
    enable_login_password_changed: qt_signal!(),

    // 是否启用单实例
    single_ins: qt_property!(bool; NOTIFY single_ins_changed),
    single_ins_changed: qt_signal!(),

    window_opacity: qt_property!(f32; NOTIFY window_opacity_changed),
    window_opacity_changed: qt_signal!(),

    window_width: qt_property!(f32; NOTIFY window_width_changed),
    window_width_changed: qt_signal!(),

    window_height: qt_property!(f32; NOTIFY window_height_changed),
    window_height_changed: qt_signal!(),

    pub price_refresh_interval: qt_property!(u32; NOTIFY price_refresh_interval_changed),
    price_refresh_interval_changed: qt_signal!(),

    pub price_item_count: qt_property!(u32; NOTIFY price_item_count_changed),
    price_item_count_changed: qt_signal!(),

    pub browser: qt_property!(QString; NOTIFY browser_changed),
    browser_changed: qt_signal!(),

    pub panel_type: qt_property!(u32; NOTIFY panel_type_changed),
    panel_type_changed: qt_signal!(),

    save_qml: qt_method!(fn(&mut self)),
}

impl Config {
    pub fn init_from_engine(engine: &mut QmlEngine, config: QObjectPinned<Config>) {
        engine.set_object_property("config".into(), config);
        qml_register_enum::<PanelType>(cstr!("PanelType"), 1, 0, cstr!("PanelType"));
    }

    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.path = app_dirs
            .config_dir
            .join("app.conf")
            .to_str()
            .unwrap()
            .to_string();

        self.config_dir = app_dirs.config_dir.to_str().unwrap().to_string().into();
        self.data_dir = app_dirs.data_dir.to_str().unwrap().to_string().into();

        self.working_dir = match env::current_exe() {
            Ok(mut dir) => {
                dir.pop();
                dir.to_str().unwrap().to_string().into()
            }
            Err(e) => {
                debug!("{:?}", e);
                "".to_string().into()
            }
        };
        self.load();
    }

    fn load(&mut self) {
        let mut raw_config = RawConfig::default();
        if let Ok(text) = std::fs::read_to_string(&self.path) {
            if let Ok(_raw_config) = serde_json::from_str::<RawConfig>(&text) {
                raw_config = _raw_config;
            }
        }

        self.font_pixel_size_normal =
            std::cmp::min(std::cmp::max(raw_config.font_pixel_size_normal, 10u32), 32);
        self.is_dark_theme = raw_config.is_dark_theme;
        self.is_window_mode = raw_config.is_window_mode;
        self.use_chinese = raw_config.use_chinese;
        self.show_splash = raw_config.show_splash;
        self.use_login_sound = raw_config.use_login_sound;
        self.enable_web_server = raw_config.enable_web_server;
        self.web_server_address = raw_config.web_server_address.into();
        self.web_server_port = raw_config.web_server_port;
        self.enable_login_password = raw_config.enable_login_password;
        self.single_ins = raw_config.single_ins;
        self.splash_interval = raw_config.splash_interval;
        self.window_opacity = f32::max(raw_config.window_opacity, 0.3);
        self.window_width = f32::max(raw_config.window_width, 840f32);
        self.window_height = f32::max(raw_config.window_height, 680f32);
        self.price_refresh_interval = raw_config.price_refresh_interval;
        self.price_item_count = raw_config.price_item_count;
        self.browser = raw_config.browser.into();
        self.panel_type = PanelType::Price as u32;
        self.owlracle_api_key = raw_config.owlracle_api_key.into();
        self.okex_api_key = raw_config.okex_api_key.into();
        self.okex_passphrase = raw_config.okex_passphrase.into();
        self.okex_secret_key = raw_config.okex_secret_key.into();
        self.okex_websocket_is_start_enable = raw_config.okex_websocket_is_start_enable;
        self.unrefresh_when_not_focus = raw_config.unrefresh_when_not_focus;
    }

    pub fn save_qml(&mut self) {
        if self.path.is_empty() {
            return;
        }

        let raw_config = RawConfig {
            font_pixel_size_normal: self.font_pixel_size_normal,
            is_dark_theme: self.is_dark_theme,
            is_window_mode: self.is_window_mode,
            use_chinese: self.use_chinese,
            show_splash: self.show_splash,
            use_login_sound: self.use_login_sound,
            enable_web_server: self.enable_web_server,
            web_server_address: self.web_server_address.to_string(),
            web_server_port: self.web_server_port,
            enable_login_password: self.enable_login_password,
            single_ins: self.single_ins,
            splash_interval: self.splash_interval,
            window_opacity: self.window_opacity,
            window_width: self.window_width,
            window_height: self.window_height,
            price_refresh_interval: self.price_refresh_interval,
            price_item_count: self.price_item_count,
            browser: self.browser.to_string(),
            owlracle_api_key: self.owlracle_api_key.to_string(),
            okex_api_key: self.okex_api_key.to_string(),
            okex_passphrase: self.okex_passphrase.to_string(),
            okex_secret_key: self.okex_secret_key.to_string(),
            okex_websocket_is_start_enable: self.okex_websocket_is_start_enable,
            unrefresh_when_not_focus: self.unrefresh_when_not_focus,
        };

        if let Ok(text) = serde_json::to_string_pretty(&raw_config) {
            if let Err(_) = std::fs::write(&self.path, text) {
                warn!("save config {:?} failed", &self.path);
            }
        }
    }
}
