use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct RawConfig {
    font_pixel_size_normal: u32,
    is_dark_theme: bool,
    use_chinese: bool,
    show_live_circle: bool,
    window_opacity: f32,
    price_refresh_interval: u32,
}

#[derive(QObject, Default)]
pub struct Config {
    base: qt_base_class!(trait QObject),
    config_path: String,

    // UI
    font_pixel_size_normal: qt_property!(u32; NOTIFY font_pixel_size_normal_changed),
    font_pixel_size_normal_changed: qt_signal!(),

    is_dark_theme: qt_property!(bool; NOTIFY is_dark_theme_changed),
    is_dark_theme_changed: qt_signal!(),

    use_chinese: qt_property!(bool; NOTIFY use_chinese_changed),
    use_chinese_changed: qt_signal!(),

    show_live_circle: qt_property!(bool; NOTIFY show_live_circle_changed WRITE set_show_live_circle),
    set_show_live_circle: qt_method!(fn(&mut self, show: bool)),
    show_live_circle_changed: qt_signal!(),

    window_opacity: qt_property!(f32; NOTIFY window_opacity_changed WRITE set_window_opacity),
    set_window_opacity: qt_method!(fn(&mut self, opacity: f32)),
    window_opacity_changed: qt_signal!(),

    price_refresh_interval: qt_property!(u32; NOTIFY price_refresh_interval_changed),
    price_refresh_interval_changed: qt_signal!(),

    save_config: qt_method!(fn(&mut self)),
}

impl Config {
    pub fn init_from_engine(engine: &mut QmlEngine, config: QObjectPinned<Config>) {
        engine.set_object_property("config".into(), config);
    }

    pub fn get_use_chinese(&self) -> bool {
        return self.use_chinese;
    }

    pub fn set_config_path(&mut self, path: &str) {
        self.config_path = path.to_string();
    }

    fn init_default_config(&mut self) {
            self.font_pixel_size_normal = 16;
            self.is_dark_theme = false;
            self.use_chinese = true;
            self.show_live_circle = false;
            self.window_opacity = 1.0;
            self.price_refresh_interval = 30;
    }

    pub fn load_config(&mut self) {
        if self.config_path.is_empty() {
            return;
        }

        self.init_default_config();
        if let Ok(text) = std::fs::read_to_string(&self.config_path) {
            if let Ok(raw_config) = serde_json::from_str::<RawConfig>(&text) {
                self.font_pixel_size_normal =
                    std::cmp::min(std::cmp::max(raw_config.font_pixel_size_normal, 10u32), 32);
                self.is_dark_theme = raw_config.is_dark_theme;
                self.use_chinese = raw_config.use_chinese;
                self.show_live_circle = raw_config.show_live_circle;
                self.window_opacity = f32::max(raw_config.window_opacity, 0.3);
                self.price_refresh_interval = raw_config.price_refresh_interval;
            }
        }
    }

    pub fn save_config(&mut self) {
        if self.config_path.is_empty() {
            return;
        }

        let raw_config = RawConfig {
            font_pixel_size_normal: self.font_pixel_size_normal,
            is_dark_theme: self.is_dark_theme,
            use_chinese: self.use_chinese,
            show_live_circle: self.show_live_circle,
            window_opacity: self.window_opacity,
            price_refresh_interval: self.price_refresh_interval,
        };

        if let Ok(text) = serde_json::to_string_pretty(&raw_config) {
            if let Err(_) = std::fs::write(&self.config_path, text) {
                warn!("save config {:?} failed", &self.config_path);
            }
        }
    }

    pub fn set_show_live_circle(&mut self, show: bool) {
        if show == self.show_live_circle {
            return;
        }

        self.show_live_circle = show;
        self.show_live_circle_changed();
    }

    pub fn set_window_opacity(&mut self, opacity: f32) {
        let mut opacity = (opacity  * 100.0).round() / 100.0;
        if opacity < 0.1 || opacity > 1.0 {
            opacity = 1.0
        }

        self.window_opacity = opacity;
        self.window_opacity_changed();
    }
}
