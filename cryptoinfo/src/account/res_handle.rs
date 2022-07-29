pub mod okex_pri {
    use super::super::okex;
    use super::super::res_parser;
    use crate::qobjmgr::{qobj_mut, NodeType};
    use crate::translator::Translator;
    #[allow(unused_imports)]
    use ::log::{debug, warn};
    use modeldata::*;

    pub fn login(qptr: QBox<okex::Account>, msg: &str) {
        let (ok, reason) = res_parser::okex_login_ok(&msg);
        qptr.borrow_mut().set_is_login(ok);
        debug!("Login OKEX pri wss: {:?}, reason: {}", ok, &reason);
    }

    pub fn error(qptr: QBox<okex::Account>, msg: &str) {
        let ts = qobj_mut::<Translator>(NodeType::Translator);
        let msg = res_parser::okex_error_msg(&msg);
        let msg = format!("{}:{}", ts.tr("出错".to_string().into()).to_string(), msg);
        qptr.borrow_mut().set_msg_tip(msg, true);
    }
}

pub mod okex_pub {
    use super::super::okex;
    use super::super::res_parser;
    use crate::qobjmgr::{qobj_mut, NodeType};
    use crate::translator::Translator;
    use crate::utility::Utility;
    #[allow(unused_imports)]
    use ::log::{debug, warn};
    use modeldata::*;

    pub fn login(qptr: QBox<okex::Account>, msg: &str) {
        let ts = qobj_mut::<Translator>(NodeType::Translator);
        let (ok, reason) = res_parser::okex_login_ok(&msg);
        qptr.borrow_mut().set_is_login(ok);

        if ok {
            qptr.borrow_mut()
                .set_msg_tip(ts.tr("登陆成功".into()).to_string(), false);
            qptr.borrow_mut().update_time = Utility::local_time_now("%H:%M:%S").into();
        } else {
            qptr.borrow_mut().set_msg_tip(
                format!("{}:{}", ts.tr("登陆失败! 原因".into()).to_string(), &reason),
                true,
            );
        };
        debug!("Login OKEX pub wss: {:?}, reason: {}", ok, &reason);
    }

    pub fn error(qptr: QBox<okex::Account>, msg: &str) {
        let ts = qobj_mut::<Translator>(NodeType::Translator);
        let msg = res_parser::okex_error_msg(&msg);
        let msg = format!("{}:{}", ts.tr("出错".to_string().into()).to_string(), msg);
        qptr.borrow_mut().set_msg_tip(msg, true);
    }
}
