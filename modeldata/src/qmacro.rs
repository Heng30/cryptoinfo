#[macro_export]
macro_rules! modeldata_struct {
    ($model: ident, $modelitem: ident,
     members: {$($v: ident: $t: ident),*$(,)*},
     members_qt: {$($sv: ident: [$st: ident; $st_n: ident]),*$(,)*},
     signals_qt: {$($sg: ident), *$(,)*},
     methods_qt: {$($mv: ident: $mt: ty),*$(,)*}) => {
        use qmetaobject::*;
        #[derive(QObject, Default)]
        pub struct $model {
            base: qt_base_class!(trait QAbstractListModel),
            count: qt_property!(i32; READ row_count NOTIFY count_changed),
            count_changed: qt_signal!(),
            updated: qt_signal!(),
            inner_model: ModelData<$model, $modelitem>,
            mutex: std::sync::Mutex<()>,

            item_qml: qt_method!(fn(&mut self, index: usize) -> QVariant),
            items_list_qml: qt_method!(fn(&mut self) -> QVariantList),

            items_len_qml: qt_method!(fn(&self) -> usize),

            items_is_empty_qml: qt_method!(fn(&self) -> bool),

            items_changed_qml: qt_method!(fn(&mut self, from: usize, to: usize)),

            insert_rows_qml: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
            remove_rows_qml: qt_method!(fn(&mut self, row: usize, count: usize) -> bool),
            swap_row_qml: qt_method!(fn(&mut self, from: usize, to: usize)),
            up_row_qml: qt_method!(fn(&mut self, index: usize)),
            down_row_qml: qt_method!(fn(&mut self, index: usize)),
            clear_qml: qt_method!(fn(&mut self)),


            $(pub $v: $t,)*
            $($mv: qt_method!($mt),)*
            $($sg: qt_signal!(),)*
            $(
                pub $sv: qt_property!($st; NOTIFY $st_n),
                pub $st_n: qt_signal!(),
            )*

        }

        impl QAbstractListModel for $model {
            fn row_count(&self) -> i32 {
                self.inner_model.items().len() as i32
            }

            fn data(&self, index: QModelIndex, role: i32) -> QVariant {
                if role != USER_ROLE {
                    return QVariant::default();
                }

                self.inner_model
                    .items()
                    .get(index.row() as usize)
                    .map(|x| x.to_qvariant())
                    .unwrap_or_default()
            }

            fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
                vec![(USER_ROLE, QByteArray::from("modelData"))]
                    .into_iter()
                    .collect()
            }
        }

        impl $model {
            pub fn init_from_engine(engine: &mut QmlEngine, model: QObjectPinned<$model>, qml_name: &str) {
                engine.set_object_property(qml_name.into(), model);
                model
                    .borrow_mut()
                    .inner_model
                    .set_parent(QBox::new(model.borrow()));
            }

            pub fn internal_init(&mut self) {
                self.inner_model.set_parent(QBox::new(self));
            }

            pub fn items(&self) -> &Vec<$modelitem> {
                return self.inner_model.items();
            }

            pub fn items_mut(&mut self) -> &mut Vec<$modelitem> {
                return self.inner_model.items_mut();
            }

            pub fn set(&mut self, index: usize, item: $modelitem) {
                self.inner_model.set(index, item);
            }

            pub fn append(&mut self, item: $modelitem) {
                self.inner_model.append(item);
                self.count_changed();
            }

            pub fn clear(&mut self) {
                self.inner_model.clear();
                self.count_changed();
            }

            pub fn insert_rows(&mut self, row: usize, count: usize) -> bool {
                let ret = self.inner_model.insert_rows(row, count);
                self.count_changed();
                return ret;
            }

            pub fn remove_rows(&mut self, row: usize, count: usize) -> bool {
                let ret = self.inner_model.remove_rows(row, count);
                self.count_changed();
                return ret;
            }

            pub fn swap_row(&mut self, from: usize, to: usize) {
                self.inner_model.swap_row(from, to);
            }

            pub fn items_changed(&mut self, from: usize, to: usize) {
                self.inner_model.data_changed(from, to);
            }

            pub fn items_len(&self) -> usize {
                return self.inner_model.items().len();
            }

            pub fn items_is_empty(&self) -> bool {
                return self.items_len() <= 0;
            }

            pub fn up_row(&mut self, index: usize) {
                if index <= 0 {
                    return;
                }
                self.swap_row(index - 1, index);
            }

            pub fn down_row(&mut self, index: usize) {
                self.swap_row(index, index + 1);
            }

            // 导出到qml的函数
            fn item_qml(&mut self, index: usize) -> QVariant {
                return self.inner_model.item(index);
            }

            fn items_list_qml(&mut self) -> QVariantList {
                return self.inner_model.item_list();
            }

            pub fn items_len_qml(&self) -> usize {
                return self.items_len();
            }

            pub fn items_is_empty_qml(&self) -> bool {
                return self.items_is_empty();
            }

            pub fn items_changed_qml(&mut self, from: usize, to: usize) {
                self.items_changed(from, to);
            }

            pub fn insert_rows_qml(&mut self, row: usize, count: usize) -> bool {
                return self.insert_rows(row, count);
            }

            pub fn remove_rows_qml(&mut self, row: usize, count: usize) -> bool {
                return self.remove_rows(row, count);
            }

            pub fn swap_row_qml(&mut self, from: usize, to: usize) {
                self.swap_row(from, to);
            }

            pub fn up_row_qml(&mut self, index: usize) {
                self.up_row(index);
            }

            pub fn down_row_qml(&mut self, index: usize) {
                self.down_row(index);
            }

            pub fn clear_qml(&mut self) {
                self.clear();
            }

        }
    };
}
