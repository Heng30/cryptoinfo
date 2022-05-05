use hotkey;
use modeldata::QBox;
use qmetaobject::*;
use tokio;
use enigo::*;

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Ghotkey {
    base: qt_base_class!(trait QObject),
    hotkey: QBox<hotkey::Listener>,
    ctrl_alt_h_pressed: qt_signal!(),
    listener_exit: qt_method!(fn(&mut self)),
}

impl Ghotkey {
    pub fn init_from_engine(engine: &mut QmlEngine, hotkey: QObjectPinned<Ghotkey>) {
        engine.set_object_property("ghotkey".into(), hotkey);
    }

    // 退出监听线程
    pub fn listener_exit(&mut self) {
        self.hotkey.get_mut().exit();
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
            hotkey.get_mut().hotkey = QBox::new(&hk);

            hk.register_hotkey(
                hotkey::modifiers::CONTROL | hotkey::modifiers::ALT,
                'H' as u32,
                move || hotkey.get_mut().ctrl_alt_h_pressed(),
            )
            .unwrap();

            hk.listen();
        });
    }
}
