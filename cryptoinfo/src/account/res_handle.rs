use super::data as AccountData;
#[allow(unused_imports)]
use ::log::{debug, warn};

pub enum OkexResMsgEventType {
    Unknown = 0,
    #[allow(dead_code)]
    Ping = 1,
    Pong = 2,
    Login = 3,
}

pub fn res_msg_event_type(msg: &str) -> OkexResMsgEventType {
    if msg == "pong" {
        return OkexResMsgEventType::Pong;
    }

    match serde_json::from_str::<AccountData::OkexResMsgEvent>(msg) {
        Ok(event) => {
            if event.event == "login" {
                return OkexResMsgEventType::Login;
            }
        }
        Err(e) => debug!("{:?}", e),
    };
    return OkexResMsgEventType::Unknown;
}

pub fn okex_login_ok(msg: &str) -> bool {
    match serde_json::from_str::<AccountData::OkexLoginResMsg>(msg) {
        Ok(event) => {
            if event.code == "0" {
                return true;
            }
        }
        Err(e) => debug!("{:?}", e),
    };
    return false;
}
