use super::data::{self, okex_req};
use super::res_handle::{self, okex_pri, okex_pub};
use super::res_parser;
use super::OkexSubStaModel;
use crate::config::Config;
use crate::qobjmgr::{qobj, qobj_mut, NodeType};
#[allow(unused_imports)]
use ::log::{debug, warn};
use futures_channel::mpsc;
use futures_util::{future, pin_mut, StreamExt};
use modeldata::*;
use qmetaobject::*;
use std::sync::Mutex;
use tokio::{self, time};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[derive(QObject, Default)]
pub struct Account {
    base: qt_base_class!(trait QObject),
    mutex: Mutex<()>,
    wss_pub_url: String,
    wss_pri_url: String,
    pri_tx: QBox<mpsc::UnboundedSender<Message>>,
    pub_tx: QBox<mpsc::UnboundedSender<Message>>,
    is_subscribe: bool,

    msg_tip_is_error: qt_property!(bool),
    msg_tip: qt_property!(QString; NOTIFY msg_tip_changed),
    msg_tip_changed: qt_signal!(),

    login_count: u32,
    is_login: qt_property!(bool; NOTIFY is_login_changed),
    is_login_changed: qt_signal!(),

    pub update_time: qt_property!(QString; NOTIFY update_time_changed),
    pub update_time_changed: qt_signal!(),

    pub break_link_qml: qt_method!(fn(&mut self)),
    pub refresh_qml: qt_method!(fn(&mut self)),
}

impl Account {
    pub fn init_from_engine(engine: &mut QmlEngine, account: QObjectPinned<Account>, name: &str) {
        engine.set_object_property(name.into(), account);
    }

    pub fn init(&mut self) {
        self.wss_pri_url = "wss://ws.okx.com:8443/ws/v5/private".into();
        self.wss_pub_url = "wss://ws.okx.com:8443/ws/v5/public".into();
        self.run();
        self.run_ping_timer();
    }

    fn run_ping_timer(&mut self) {
        let mut second = time::interval(time::Duration::from_secs(10));
        let qptr = QBox::new(self);

        tokio::spawn(async move {
            loop {
                {
                    let _ = qptr.borrow_mut().mutex.lock().unwrap();
                    if qptr.borrow().is_login {
                        qptr.borrow_mut().send_pri_msg("ping".to_string());
                        qptr.borrow_mut().send_pub_msg("ping".to_string());
                    }
                }
                second.tick().await;
            }
        });
    }

    pub fn set_msg_tip(&mut self, msg: String, is_error: bool) {
        let _ = self.mutex.lock().unwrap();
        self.msg_tip_is_error = is_error;
        self.msg_tip = msg.into();
        self.msg_tip_changed();
    }

    pub fn set_is_login(&mut self, is_ok: bool) {
        let _ = self.mutex.lock().unwrap();
        if is_ok {
            self.login_count += 1;
            if self.login_count >= 2 {
                self.is_login = is_ok;
                self.is_login_changed();
            }
        } else {
            self.login_count = 0;
            self.is_login = is_ok;
            self.is_login_changed();
        }
    }

    pub fn subscribe(&mut self) {
        let _ = self.mutex.lock().unwrap();
        if !self.is_login || self.is_subscribe {
            return;
        }
        debug!("start subscribe...");
        let sub = qobj_mut::<OkexSubStaModel>(NodeType::OkexSubStaModel);
        sub.subscribe_only_channel(self);
        self.is_subscribe = true;
        debug!("subscribe finished...");
    }

    fn recv_pri_msg(qptr: QBox<Account>, msg: String) {
        if msg == "pong" {
            return;
        }
        debug!("recv pri msg: {}", &msg);
        match res_parser::okex::event_type(&msg) {
            res_parser::okex::MsgEventType::Login => {
                res_handle::okex::login(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Error => {
                res_handle::okex::error(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Subscribe => {
                res_handle::okex::subscribe(qptr, &msg);
            }
            _ => return,
        }
    }

    fn recv_pub_msg(qptr: QBox<Account>, msg: String) {
        if msg == "pong" {
            return;
        }
        debug!("recv pub msg: {}", &msg);
        match res_parser::okex::event_type(&msg) {
            res_parser::okex::MsgEventType::Login => {
                res_handle::okex::login(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Error => {
                res_handle::okex::error(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Subscribe => {
                res_handle::okex::subscribe(qptr, &msg);
            }
            _ => return,
        }
    }

    pub fn send_pri_msg(&mut self, msg: String) {
        let _ = self.mutex.lock().unwrap();
        if self.pri_tx.is_null() || self.pri_tx.borrow_mut().is_closed() {
            debug!("pri_tx can not send msg.");
            return;
        }
        if msg != "ping" {
            debug!("send pri msg: {}", &msg);
        }
        match self.pri_tx.borrow_mut().unbounded_send(Message::Text(msg)) {
            Ok(_) => (),
            Err(e) => debug!("{:?}", e),
        }
    }

    pub fn send_pub_msg(&mut self, msg: String) {
        let _ = self.mutex.lock().unwrap();
        if self.pub_tx.is_null() || self.pub_tx.borrow_mut().is_closed() {
            debug!("pub_tx can not send msg.");
            return;
        }
        if msg != "ping" {
            debug!("send pri msg: {}", &msg);
        }
        match self.pub_tx.borrow_mut().unbounded_send(Message::Text(msg)) {
            Ok(_) => (),
            Err(e) => debug!("{:?}", e),
        }
    }

    fn login(&mut self) {
        let config = qobj::<Config>(NodeType::Config);
        let msg = okex_req::LoginMsg::new(
            &config.okex_passphrase.to_string(),
            &config.okex_api_key.to_string(),
            &config.okex_secret_key.to_string(),
        );
        self.send_pri_msg(msg.clone());
        self.send_pub_msg(msg);
    }

    fn run(&mut self) {
        let config = qobj::<Config>(NodeType::Config);
        if config.okex_api_key.is_empty()
            || config.okex_passphrase.is_empty()
            || config.okex_secret_key.is_empty()
        {
            debug!(
                "invalid okex login info: key: {}, passphrase: {}",
                &config.okex_api_key.to_string(),
                &config.okex_passphrase.to_string()
            );
            return;
        }

        let pri_url = Url::parse(&self.wss_pri_url).unwrap();
        let pub_url = Url::parse(&self.wss_pub_url).unwrap();
        let qptr = QBox::new(self);

        tokio::spawn(async move {
            let pri_stream = match connect_async(pri_url).await {
                Ok((stream, _)) => stream,
                Err(e) => {
                    debug!("{:?}", e);
                    return;
                }
            };

            let pub_stream = match connect_async(pub_url).await {
                Ok((stream, _)) => stream,
                Err(e) => {
                    debug!("{:?}", e);
                    return;
                }
            };

            debug!("WebSocket handshake has been successfully completed");

            let (pri_writer, pri_reader) = pri_stream.split();
            let (pub_writer, pub_reader) = pub_stream.split();
            let pri_channel = Box::new(mpsc::unbounded::<Message>());
            let pub_channel = Box::new(mpsc::unbounded::<Message>());
            let forward2pri = pri_channel.1.map(Ok).forward(pri_writer);
            let forword2pub = pub_channel.1.map(Ok).forward(pub_writer);

            {
                let _ = qptr.borrow_mut().mutex.lock().unwrap();
                qptr.borrow_mut().pri_tx = QBox::new(&pri_channel.0);
                qptr.borrow_mut().pub_tx = QBox::new(&pub_channel.0);
            }

            let handle_pri_msg = {
                pri_reader.for_each(|message| async {
                    if let Ok(data) = message {
                        match data.into_text() {
                            Ok(text) => Self::recv_pri_msg(qptr, text),
                            Err(e) => debug!("{:?}", e),
                        }
                    }
                })
            };

            let handle_pub_msg = {
                pub_reader.for_each(|message| async {
                    if let Ok(data) = message {
                        match data.into_text() {
                            Ok(text) => Self::recv_pub_msg(qptr, text),
                            Err(e) => debug!("{:?}", e),
                        }
                    }
                })
            };

            qptr.borrow_mut().login();
            pin_mut!(forward2pri, forword2pub, handle_pri_msg, handle_pub_msg);
            future::select(
                future::select(handle_pri_msg, handle_pub_msg),
                future::select(forward2pri, forword2pub),
            )
            .await;
            qptr.borrow_mut().break_link_qml();
            qptr.borrow_mut().set_is_login(false);
            debug!("OKEX websocket exit...");
        });
    }

    fn break_link_qml(&mut self) {
        let _ = self.mutex.lock().unwrap();
        if !self.pri_tx.is_null() && !self.pri_tx.borrow_mut().is_closed() {
            self.pri_tx.borrow_mut().close_channel();
            self.pri_tx = QBox::default();
        }

        if !self.pub_tx.is_null() && !self.pub_tx.borrow_mut().is_closed() {
            self.pub_tx.borrow_mut().close_channel();
            self.pub_tx = QBox::default();
        }

        self.is_subscribe = false;
        let sub = qobj_mut::<OkexSubStaModel>(NodeType::OkexSubStaModel);
        sub.offline();
    }

    fn refresh_qml(&mut self) {
        self.break_link_qml();
        self.run();
    }
}
