use super::data::okex_req;
use super::res_handle;
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
use std::sync::atomic::{AtomicBool, Ordering as AOrdering};
use std::sync::Mutex;
use tokio::{self, time};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

type MUSender = Mutex<Option<mpsc::UnboundedSender<Message>>>;

#[derive(QObject, Default)]
pub struct Account {
    base: qt_base_class!(trait QObject),
    wss_pub_url: String,
    wss_pri_url: String,
    pri_tx: MUSender,
    pub_tx: MUSender,

    is_subscribe: AtomicBool,
    is_login: AtomicBool,
    is_login_changed: qt_signal!(),

    pub update_time: qt_property!(QString; NOTIFY update_time_changed),
    pub update_time_changed: qt_signal!(),

    pub break_link_qml: qt_method!(fn(&mut self)),
    pub refresh_qml: qt_method!(fn(&mut self)),

    is_login_qml: qt_method!(fn(&self) -> bool),
}

impl Account {
    pub fn init_from_engine(engine: &mut QmlEngine, account: QObjectPinned<Account>, name: &str) {
        engine.set_object_property(name.into(), account);
    }

    pub fn init(&mut self) {
        let config = qobj::<Config>(NodeType::Config);
        self.wss_pri_url = "wss://ws.okx.com:8443/ws/v5/private".into();
        self.wss_pub_url = "wss://ws.okx.com:8443/ws/v5/public".into();
        if config.okex_websocket_is_start_enable {
            self.run();
        }
        self.run_ping_timer();
    }

    fn run_ping_timer(&mut self) {
        let mut second = time::interval(time::Duration::from_secs(10));
        let qptr = QBox::new(self);

        tokio::spawn(async move {
            loop {
                if qptr.borrow().is_login.load(AOrdering::SeqCst) {
                    qptr.borrow_mut().send_pri_msg("ping".to_string());
                    qptr.borrow_mut().send_pub_msg("ping".to_string());
                }
                second.tick().await;
            }
        });
    }

    pub fn set_is_login(&mut self, is_ok: bool) {
        if self.is_login.load(AOrdering::SeqCst) == is_ok {
            return;
        }

        self.is_login.store(is_ok, AOrdering::SeqCst);
        self.is_login_changed();
    }

    pub fn subscribe(&mut self) {
        if !self.is_login.load(AOrdering::SeqCst) || self.is_subscribe.load(AOrdering::SeqCst) {
            return;
        }

        debug!("start subscribe...");
        let sub = qobj_mut::<OkexSubStaModel>(NodeType::OkexSubStaModel);
        sub.subscribe_channel(self);
        self.is_subscribe.store(true, AOrdering::SeqCst);
        debug!("subscribe finished...");
    }

    pub fn recv_pri_msg(qptr: QBox<Account>, msg: String) {
        if msg == "pong" {
            return;
        }

        if msg.len() < 512 {
            debug!("recv pri msg: {}", &msg);
        }
        match res_parser::okex::event_type(&msg) {
            res_parser::okex::MsgEventType::Login => {
                res_handle::okex::login(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Error => {
                res_handle::okex::error(&msg);
            }
            res_parser::okex::MsgEventType::Subscribe => {
                res_handle::okex::subscribe(qptr, &msg);
            }
            _ => match res_parser::okex::channel_type(&msg) {
                res_parser::okex::MsgChannelType::Account => {
                    res_handle::okex_pri::account_channel(qptr, &msg);
                }
                res_parser::okex::MsgChannelType::Position => {
                    res_handle::okex_pri::position_channel(qptr, &msg);
                }
                res_parser::okex::MsgChannelType::Greek => {
                    res_handle::okex_pri::greek_channel(qptr, &msg);
                }
                _ => (),
            },
        }
    }

    pub fn recv_pub_msg(qptr: QBox<Account>, msg: String) {
        if msg == "pong" {
            return;
        }

        if msg.len() < 512 {
            debug!("recv pub msg: {}", &msg);
        }

        match res_parser::okex::event_type(&msg) {
            res_parser::okex::MsgEventType::Login => {
                res_handle::okex::login(qptr, &msg);
            }
            res_parser::okex::MsgEventType::Error => {
                res_handle::okex::error(&msg);
            }
            res_parser::okex::MsgEventType::Subscribe => {
                res_handle::okex::subscribe(qptr, &msg);
            }
            _ => return,
        }
    }

    pub fn send_pri_msg(&mut self, msg: String) {
        let mut pri_tx = self.pri_tx.lock().unwrap();
        if pri_tx.is_none() {
            debug!("pri_tx can not send msg.");
            return;
        }
        if msg != "ping" {
            debug!("send pri msg: {}", &msg);
        }

        let pri_tx = pri_tx.as_mut().unwrap();
        match pri_tx.unbounded_send(Message::Text(msg)) {
            Ok(_) => (),
            Err(e) => debug!("{:?}", e),
        }
    }

    pub fn send_pub_msg(&mut self, msg: String) {
        let mut pub_tx = self.pub_tx.lock().unwrap();
        if pub_tx.is_none() {
            debug!("pub_tx can not send msg.");
            return;
        }
        if msg != "ping" {
            debug!("send pub msg: {}", &msg);
        }

        let pub_tx = pub_tx.as_mut().unwrap();
        match pub_tx.unbounded_send(Message::Text(msg)) {
            Ok(_) => (),
            Err(e) => debug!("{:?}", e),
        }
    }

    fn login(&mut self) {
        // Note: editting the okex configure, when run this function in other thread is unsafe.
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
            let pri_channel = mpsc::unbounded::<Message>();
            let pub_channel = mpsc::unbounded::<Message>();
            let forward2pri = pri_channel.1.map(Ok).forward(pri_writer);
            let forword2pub = pub_channel.1.map(Ok).forward(pub_writer);

            qptr.borrow_mut().pri_tx = Mutex::new(Some(pri_channel.0));
            qptr.borrow_mut().pub_tx = Mutex::new(Some(pub_channel.0));

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
            debug!("OKEX websocket exit...");
        });
    }

    fn break_link_qml(&mut self) {
        self.pri_tx.lock().unwrap().take();
        self.pub_tx.lock().unwrap().take();
        self.set_is_login(false);
        self.is_subscribe.store(false, AOrdering::SeqCst);
        qobj_mut::<OkexSubStaModel>(NodeType::OkexSubStaModel).offline();
    }

    fn refresh_qml(&mut self) {
        self.break_link_qml();
        self.run();
    }

    fn is_login_qml(&self) -> bool {
        self.is_login.load(AOrdering::SeqCst)
    }
}
