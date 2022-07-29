use super::data as AccountData;
#[allow(unused_imports)]
use ::log::{debug, warn};

pub enum OkexResMsgEventType {
    Unknown = 0,
    #[allow(dead_code)]
    Ping = 1,
    Pong = 2,
    Login = 3,
    Error = 4,
}

pub fn res_msg_event_type(msg: &str) -> OkexResMsgEventType {
    if msg == "pong" {
        return OkexResMsgEventType::Pong;
    }

    match serde_json::from_str::<AccountData::OkexResMsgEvent>(msg) {
        Ok(event) => {
            if event.event == "login" {
                return OkexResMsgEventType::Login;
            } else if event.event == "error" {
                return OkexResMsgEventType::Error;
            }
        }
        Err(e) => debug!("{:?}", e),
    };
    return OkexResMsgEventType::Unknown;
}

pub fn okex_login_ok(msg: &str) -> (bool, String) {
    match serde_json::from_str::<AccountData::OkexLoginResMsg>(msg) {
        Ok(event) => {
            if event.code == "0" {
                return (true, event.msg);
            } else {
                return (false, event.msg);
            }
        }
        Err(e) => {
            debug!("{:?}", &e);
            return (false, format!("{:?}", e));
        }
    };
}

pub fn okex_error_msg(msg: &str) -> String {
    match serde_json::from_str::<AccountData::OkexLoginResMsg>(msg) {
        Ok(event) => {
            return event.msg;
        }
        Err(e) => {
            debug!("{:?}", &e);
            return format!("{:?}", e);
        }
    };
}
