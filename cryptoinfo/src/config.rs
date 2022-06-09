use crate::qobjmgr::{qobj, NodeType as QNodeType};
use cstr::cstr;
use platform_dirs::AppDirs;
use qmetaobject::*;
use serde_derive::{Deserialize, Serialize};

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(Copy, Clone, Debug, Eq, PartialEq, QEnum)]
#[repr(C)]
enum PanelType {
    Price = 1,
    Setting = 2,
    Note = 3,
    Todo = 4,
    ToolBox = 5,
    DefiProtocol = 6,
    DefiChain = 7,
    DefiChart = 8,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawConfig {
    font_pixel_size_normal: u32,
    is_dark_theme: bool,
    is_window_mode: bool,
    use_chinese: bool,
    show_splash: bool,
    use_login_sound: bool,
    enable_login_password: bool,
    single_ins: bool,
    splash_interval: u32,
    window_opacity: f32,
    window_width: f32,
    window_height: f32,

    price_refresh_interval: u32, // 数据刷新时间间隔
    price_item_count: u32,       // 价格条目数量

    defi_refresh_interval: u32,
    defi_item_count: u32,
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
            enable_login_password: false,
            single_ins: false,
            splash_interval: 3000,
            window_opacity: 1.0,
            window_width: 840f32,
            window_height: 680f32,
            price_refresh_interval: 30,
            price_item_count: 100,
            defi_refresh_interval: 3600,
            defi_item_count: 100,
        };
    }
}

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),
    path: String,
    config_dir: qt_property!(QString),
    data_dir: qt_property!(QString),

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

    pub defi_refresh_interval: qt_property!(u32; NOTIFY defi_refresh_interval_changed),
    defi_refresh_interval_changed: qt_signal!(),

    pub defi_item_count: qt_property!(u32; NOTIFY defi_item_count_changed),
    defi_item_count_changed: qt_signal!(),

    panel_type: qt_property!(u32; NOTIFY panel_type_changed),
    panel_type_changed: qt_signal!(),

    save: qt_method!(fn(&mut self)),
}

impl Config {
    pub fn init_from_engine(engine: &mut QmlEngine, config: QObjectPinned<Config>) {
        engine.set_object_property("config".into(), config);
        qml_register_enum::<PanelType>(cstr!("PanelType"), 1, 0, cstr!("PanelType"));
    }

    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::APPDIR);
        self.path = app_dirs
            .config_dir
            .join("app.conf")
            .to_str()
            .unwrap()
            .to_string();

        self.config_dir = app_dirs.config_dir.to_str().unwrap().to_string().into();
        self.data_dir = app_dirs.data_dir.to_str().unwrap().to_string().into();
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
        self.enable_login_password = raw_config.enable_login_password;
        self.single_ins = raw_config.single_ins;
        self.splash_interval = raw_config.splash_interval;
        self.window_opacity = f32::max(raw_config.window_opacity, 0.3);
        self.window_width = f32::max(raw_config.window_width, 840f32);
        self.window_height = f32::max(raw_config.window_height, 680f32);
        self.price_refresh_interval = raw_config.price_refresh_interval;
        self.price_item_count = raw_config.price_item_count;
        self.defi_refresh_interval = raw_config.defi_refresh_interval;
        self.defi_item_count = raw_config.defi_item_count;
        self.panel_type = PanelType::Price as u32;
    }

    pub fn save(&mut self) {
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
            enable_login_password: self.enable_login_password,
            single_ins: self.single_ins,
            splash_interval: self.splash_interval,
            window_opacity: self.window_opacity,
            window_width: self.window_width,
            window_height: self.window_height,
            price_refresh_interval: self.price_refresh_interval,
            price_item_count: self.price_item_count,
            defi_refresh_interval: self.defi_refresh_interval,
            defi_item_count: self.defi_item_count,
        };

        if let Ok(text) = serde_json::to_string_pretty(&raw_config) {
            if let Err(_) = std::fs::write(&self.path, text) {
                warn!("save config {:?} failed", &self.path);
            }
        }
    }
}
