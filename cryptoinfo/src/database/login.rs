use crate::qobjmgr::{qobj, NodeType as QNodeType};
use platform_dirs::AppDirs;
use qmetaobject::*;
use rusqlite::{params, Connection};

#[allow(unused_imports)]
use ::log::{debug, warn};

#[derive(QObject, Default)]
pub struct Table {
    base: qt_base_class!(trait QObject),
    path: String,

    set_password_qml: qt_method!(fn(&mut self, password: QString) -> bool),
    del_password_qml: qt_method!(fn(&mut self, password: QString) -> bool),
    auth_qml: qt_method!(fn(&mut self, password: QString) -> bool),
}

impl Table {
    pub fn init_from_engine(engine: &mut QmlEngine, table: QObjectPinned<Table>) {
        engine.set_object_property("login_table".into(), table);
    }

    pub fn init(&mut self) {
        let app_dirs = qobj::<AppDirs>(QNodeType::AppDir);
        self.path = app_dirs
            .data_dir
            .join("cryptoinfo.db")
            .to_str()
            .unwrap()
            .to_string();

        let db = Connection::open(&self.path).unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS login (
            id    INTEGER PRIMARY KEY,
            password  TEXT NOT NULL
        )",
            [],
        )
        .unwrap();
    }

    fn set_password_qml(&mut self, password: QString) -> bool {
        if let Ok(db) = Connection::open(&self.path) {
            return db
                .execute(
                    "INSERT INTO login (password) VALUES (?1)",
                    params![&password.to_string()],
                )
                .is_ok();
        }
        return false;
    }

    fn del_password_qml(&mut self, password: QString) -> bool {
        match Connection::open(&self.path) {
            Ok(db) => {
                return db
                    .execute(
                        "DELETE FROM login WHERE `password`=(?1)",
                        params![&password.to_string()],
                    )
                    .is_ok();
            }
            Err(err) => debug!("del password failed! error: {:?}", err),
        }
        return false;
    }

    fn auth_qml(&mut self, password: QString) -> bool {
        if let Ok(db) = Connection::open(&self.path) {
            if let Ok(mut stmt) = db.prepare("SELECT id, password FROM login") {
                if let Ok(item_iter) =
                    stmt.query_map([], |row| Ok(row.get(1).unwrap_or(String::default())))
                {
                    let mut count = 0;
                    for item in item_iter {
                        count += 1;
                        let item = item.unwrap();
                        if item.is_empty() {
                            continue;
                        }

                        if item == password.to_string() {
                            return true;
                        }
                    }

                    return count == 0;
                }
            }
        }
        return true;
    }
}
