use crate::qobjmgr::{self, qobj, NodeType};
use futures_channel::mpsc;
use log::debug;
use qmetaobject::prelude::*;
use qmetaobject::QObjectPinned;
use std::sync::Mutex;

lazy_static! {
    pub static ref CHAN: Mutex<(mpsc::Sender<String>, mpsc::Receiver<String>)> =
        Mutex::new(mpsc::channel::<String>(1024));
}

#[derive(QObject, Default)]
pub struct DebugLog {
    base: qt_base_class!(trait QObject),

    pub enable: qt_property!(bool; NOTIFY enable_changed),
    enable_changed: qt_signal!(),

    text: qt_property!(QString; NOTIFY text_changed),
    text_changed: qt_signal!(),

    updated: qt_signal!(),
    recv_qml: qt_method!(fn(&mut self)),

    clear: qt_signal!(),
    clear_qml: qt_method!(fn(&mut self)),
}

impl DebugLog {
    pub fn init_from_engine(engine: &mut QmlEngine, note: QObjectPinned<DebugLog>) {
        engine.set_object_property("debug_log".into(), note);
    }

    pub fn init(&mut self) {
        match std::env::var("RUST_LOG") {
            Ok(res) => {
                if !res.is_empty() {
                    self.enable = true;
                    self.enable_changed();
                    debug!("RUST_LOG={}", &res);
                }
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    pub fn send(msg: String) {
        if !qobjmgr::contain_obj(NodeType::DebugLog) {
            return;
        }

        let _ = CHAN.lock().unwrap().0.try_send(msg);
        let debuglog = qobj::<DebugLog>(NodeType::DebugLog);
        debuglog.updated();
    }

    fn recv_qml(&mut self) {
        loop {
            match CHAN.lock().unwrap().1.try_next() {
                Ok(res) => match res {
                    Some(msg) => {
                        self.text = msg.into();
                        self.text_changed();
                    }
                    _ => return,
                },
                _ => return,
            }
        }
    }

    fn clear_qml(&mut self) {
        self.clear();
    }
}
