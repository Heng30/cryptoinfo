pub mod okex {
    use super::super::data::okex_res;
    #[allow(unused_imports)]
    use ::log::{debug, warn};

    pub enum MsgEventType {
        Unknown = 0,
        #[allow(dead_code)]
        Ping = 1,
        Pong = 2,
        Login = 3,
        Error = 4,
        Subscribe = 5,
    }

    pub enum MsgChannelType {
        Unknown = 0,
        Account = 1,
        Position = 2,
    }

    pub fn event_type(msg: &str) -> MsgEventType {
        if msg == "pong" {
            return MsgEventType::Pong;
        }

        match serde_json::from_str::<okex_res::MsgEvent>(msg) {
            Ok(event) => {
                if event.event == "login" {
                    return MsgEventType::Login;
                } else if event.event == "error" {
                    return MsgEventType::Error;
                } else if event.event == "subscribe" {
                    return MsgEventType::Subscribe;
                }
            }
            _ => (),
        };
        return MsgEventType::Unknown;
    }

    pub fn login_ok(msg: &str) -> (bool, String) {
        match serde_json::from_str::<okex_res::LoginMsg>(msg) {
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

    pub fn error(msg: &str) -> String {
        match serde_json::from_str::<okex_res::LoginMsg>(msg) {
            Ok(event) => {
                return event.msg;
            }
            Err(e) => {
                debug!("{:?}", &e);
                return format!("{:?}", e);
            }
        };
    }

    pub fn channel_type(msg: &str) -> MsgChannelType {
        match serde_json::from_str::<okex_res::MsgChannel>(msg) {
            Ok(res) => {
                if res.arg.channel == "account" {
                    return MsgChannelType::Account;
                } else if res.arg.channel == "positions" {
                    return MsgChannelType::Position;
                }
            }
            _ => (),
        };
        return MsgChannelType::Unknown;

    }

}
