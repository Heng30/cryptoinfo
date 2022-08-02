pub use okex::okex_pri;
pub use okex::okex_pub;

pub mod okex {
    use super::super::data::okex_res;
    use super::super::okex;
    use super::super::res_parser;
    use super::super::OkexSubStaModel;
    use crate::qobjmgr::{qobj_mut, NodeType};
    use crate::translator::Translator;
    use crate::utility::Utility;
    #[allow(unused_imports)]
    use ::log::{debug, warn};
    use modeldata::*;

    pub fn error(qptr: QBox<okex::Account>, msg: &str) {
        let ts = qobj_mut::<Translator>(NodeType::Translator);
        let msg = res_parser::okex::error(&msg);
        let msg = format!("{}:{}", ts.tr("出错".to_string().into()).to_string(), msg);
        qptr.borrow_mut().set_msg_tip(msg, true);
    }

    pub fn login(qptr: QBox<okex::Account>, msg: &str) {
        let ts = qobj_mut::<Translator>(NodeType::Translator);
        let (ok, reason) = res_parser::okex::login_ok(&msg);
        qptr.borrow_mut().set_is_login(ok);

        if ok {
            qptr.borrow_mut()
                .set_msg_tip(ts.tr("登陆成功".into()).to_string(), false);
            qptr.borrow_mut().update_time = Utility::local_time_now("%H:%M:%S").into();
            qptr.borrow_mut().subscribe();
        } else {
            qptr.borrow_mut().set_msg_tip(
                format!("{}:{}", ts.tr("登陆失败! 原因".into()).to_string(), &reason),
                true,
            );
        };
        debug!("Login OKEX: {:?}, reason: {}", ok, &reason);
    }

    pub fn subscribe(_qptr: QBox<okex::Account>, msg: &str) {
        match serde_json::from_str::<okex_res::Subscribe>(msg) {
            Ok(res) => {
                let model = qobj_mut::<OkexSubStaModel>(NodeType::OkexSubStaModel);
                model.add_tmp_raw_item(res.arg.channel.clone(), true);
                debug!("subscribe successfully! channel: {}", res.arg.channel);
            }
            Err(e) => {
                debug!("{:?}", &e);
            }
        };
    }

    pub mod okex_pri {
        use super::super::super::{OkexAccChanModel, OkexPosChanModel};
        use super::okex;
        use super::okex_res;
        use super::{qobj_mut, NodeType};
        #[allow(unused_imports)]
        use ::log::{debug, warn};
        use modeldata::*;

        pub fn account_channel(_qptr: QBox<okex::Account>, msg: &str) {
            match serde_json::from_str::<okex_res::AccountChannel>(msg) {
                Ok(res) => {
                    if res.data.is_empty() {
                        return;
                    }

                    let item = &res.data[0];
                    let model = qobj_mut::<OkexAccChanModel>(NodeType::OkexAccChanModel);
                    model.set_account_state(
                        item.total_eq.clone(),
                        item.iso_eq.clone(),
                        item.utime.clone(),
                    );
                    model.add_tmp_items(&item.details);
                }
                Err(e) => {
                    debug!("{:?}", &e);
                    debug!("{:?}", &msg);
                }
            };
        }

        pub fn position_channel(_qptr: QBox<okex::Account>, msg: &str) {
            match serde_json::from_str::<okex_res::PositionChannel>(msg) {
                Ok(res) => {
                    if res.data.is_empty() {
                        return;
                    }

                    let model = qobj_mut::<OkexPosChanModel>(NodeType::OkexPosChanModel);
                    model.add_tmp_items(&res.data);
                }
                Err(e) => {
                    debug!("{:?}", &e);
                    debug!("{:?}", &msg);
                }
            };
        }
    }

    pub mod okex_pub {
        // use super::super::okex;
        // use super::super::res_parser;
        // use crate::qobjmgr::{qobj_mut, NodeType};
        // use crate::translator::Translator;
        // use crate::utility::Utility;
        // #[allow(unused_imports)]
        // use ::log::{debug, warn};
        // use modeldata::*;
    }
}
