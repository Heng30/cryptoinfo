use enigo::*;
use modeldata::QBox;
use qmetaobject::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Ghotkey {
    base: qt_base_class!(trait QObject),
    hotkey: QBox<hotkey::Listener>,
    show_window: qt_property!(bool; NOTIFY show_window_changed),
    show_window_changed: qt_signal!(),

    listener_exit_qml: qt_method!(fn(&mut self)),
}

impl Ghotkey {
    pub fn init_from_engine(engine: &mut QmlEngine, hotkey: QObjectPinned<Ghotkey>) {
        engine.set_object_property("ghotkey".into(), hotkey);
    }

    // 退出监听线程
    pub fn listener_exit_qml(&mut self) {
        self.hotkey.borrow_mut().exit();
        let mut enigo = Enigo::new();

        // 模拟一次按键，触发listener监听的hotkey，才能安全退出
        enigo.key_down(Key::Control);
        enigo.key_down(Key::Alt);
        enigo.key_down(Key::Layout('H'));
        enigo.key_up(Key::Control);
        enigo.key_up(Key::Alt);
        enigo.key_up(Key::Layout('H'));
    }

    // 进行全局按键监听
    pub fn listen(hotkey: QBox<Ghotkey>) {
        tokio::spawn(async move {
            let mut hk = hotkey::Listener::new();
            hotkey.borrow_mut().hotkey = QBox::new(&hk);
            hotkey.borrow_mut().show_window = true;

            hk.register_hotkey(
                hotkey::modifiers::CONTROL | hotkey::modifiers::ALT,
                'H' as u32,
                move || hotkey.borrow_mut().show_window_changed(),
            )
            .unwrap();

            hk.listen();
        });
    }
}
