use chrono::{FixedOffset, Local, TimeZone};
use qmetaobject::*;

#[derive(QObject, Default)]
pub struct Utility {
    base: qt_base_class!(trait QObject),

    local_time_now: qt_method!(fn(&mut self, format: QString) -> QString),
    get_time_from_utc_seconds: qt_method!(fn(&self, sec: i64) -> QString),
}

impl Utility {
    pub fn init_from_engine(engine: &mut QmlEngine, utility: QObjectPinned<Utility>) {
        engine.set_object_property("utility".into(), utility);
    }

    pub fn local_time_now(&mut self, format: QString) -> QString {
        return format!(
            "{}",
            Local::now().format(format.to_string().as_str()).to_string()
        )
        .into();
    }

    pub fn get_time_from_utc_seconds(&self, sec: i64) -> QString {
        let time = FixedOffset::east(8 * 3600).timestamp(sec, 0);
        return format!("{}", time.format("%m-%d %H:%M").to_string()).into();
    }
}