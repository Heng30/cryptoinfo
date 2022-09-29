use super::QBox;
use qmetaobject::*;

#[derive(Default)]
pub struct Model<P, T>
where
    P: QObject + QAbstractListModel,
    T: 'static + QGadget + Clone,
{
    parent: QBox<P>,
    pub data: Vec<T>,
}

impl<P, T> Model<P, T>
where
    P: QObject + QAbstractListModel,
    T: 'static + QGadget + Clone + Default,
{
    pub fn set_parent(&mut self, parent: QBox<P>) {
        self.parent = parent;
    }

    // 清空model
    pub fn clear(&mut self) {
        let parent: &mut P = self.parent.borrow_mut();
        (parent as &mut dyn QAbstractListModel).begin_reset_model();
        self.data = vec![];
        (parent as &mut dyn QAbstractListModel).end_reset_model();
    }

    // 插入行
    pub fn insert_rows(&mut self, row: usize, count: usize) -> bool {
        if count == 0 || row > self.data.len() {
            return false;
        }

        let parent = self.parent.borrow_mut();
        (parent as &mut dyn QAbstractListModel)
            .begin_insert_rows(row as i32, (row + count - 1) as i32);
        for i in 0..count {
            self.data.insert(row + i, T::default());
        }
        (parent as &mut dyn QAbstractListModel).end_insert_rows();
        true
    }

    // 删除行
    pub fn remove_rows(&mut self, row: usize, count: usize) -> bool {
        if count == 0 || row + count > self.data.len() {
            return false;
        }

        let parent = self.parent.borrow_mut();
        (parent as &mut dyn QAbstractListModel)
            .begin_remove_rows(row as i32, (row + count - 1) as i32);
        self.data.drain(row..row + count);
        (parent as &mut dyn QAbstractListModel).end_remove_rows();
        true
    }

    // 交换行
    pub fn swap_row(&mut self, from: usize, to: usize) {
        if std::cmp::max(from, to) >= self.data.len() {
            return;
        }
        self.data.swap(from, to);

        let parent = self.parent.borrow_mut();
        let idx = (parent as &mut dyn QAbstractListModel).row_index(from as i32);
        (parent as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);

        let idx = (parent as &mut dyn QAbstractListModel).row_index(to as i32);
        (parent as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    // 添加条目
    pub fn append(&mut self, item: T) {
        let end = self.data.len();
        let parent = self.parent.borrow_mut();
        (parent as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);

        self.data.insert(end, item);
        (parent as &mut dyn QAbstractListModel).end_insert_rows();
    }

    // 修改条目
    pub fn set(&mut self, index: usize, item: T) {
        if index >= self.data.len() {
            return;
        }

        let parent = self.parent.borrow_mut();
        self.data[index] = item;
        let idx = (parent as &mut dyn QAbstractListModel).row_index(index as i32);
        (parent as &mut dyn QAbstractListModel).data_changed(idx.clone(), idx);
    }

    pub fn set_all(&mut self, items: Vec<T>) {
        let len = items.len();
        let olen = self.data.len();
        if len < olen {
            self.remove_rows(len, olen - len);
        } else if len > olen {
            self.insert_rows(olen, len - olen);
        }

        assert!(len == self.data.len());

        for (i, item) in items.into_iter().enumerate() {
            self.data[i] = item;
        }

        self.data_changed(0, len);
    }

    pub fn data_changed(&mut self, from: usize, to: usize) {
        let to = usize::min(to, self.data.len());
        if from > to {
            return;
        }

        let parent = self.parent.borrow_mut();
        let idx1 = (parent as &mut dyn QAbstractListModel).row_index(from as i32);
        let idx2 = (parent as &mut dyn QAbstractListModel).row_index(to as i32);
        (parent as &mut dyn QAbstractListModel).data_changed(idx1, idx2);
    }

    // 获取单个item
    pub fn item(&mut self, index: usize) -> QVariant {
        return self
            .data
            .get(index)
            .map(|x| x.to_qvariant())
            .unwrap_or(T::default().to_qvariant());
    }

    // 获取所有item
    pub fn item_list(&mut self) -> QVariantList {
        let mut list = QVariantList::default();
        for i in 0..self.data.len() {
            list.push(self.item(i));
        }

        return list;
    }

    // 获取所有item
    pub fn items(&self) -> &Vec<T> {
        return &self.data;
    }

    // 获取所有item
    pub fn items_mut(&mut self) -> &mut Vec<T> {
        return &mut self.data;
    }
}
