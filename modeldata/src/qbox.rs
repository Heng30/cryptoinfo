pub struct QBox<T> {
    v0: *const T,
    v1: *mut T,
}

impl<T> Default for QBox<T> {
    fn default() -> QBox<T> {
        return QBox {
            v0: std::ptr::null::<T>(),
            v1: std::ptr::null_mut::<T>(),
        };
    }
}

#[allow(unused)]
impl<T> QBox<T> {
    pub fn new(v: &T) -> Self {
        let v0 = v as *const T;
        let v1 = v as *const T as *mut T;
        Self { v0, v1 }
    }

    pub fn is_null(&self) -> bool {
        return self.v0.is_null() || self.v1.is_null();
    }

    pub fn get_ptr(&self) -> &*const T {
        &self.v0
    }

    pub fn get_mut_ptr(&self) -> &*mut T {
        &self.v1
    }

    pub fn get(&self) -> &T {
        assert!(!self.is_null());
        unsafe { &*std::mem::transmute::<*const T, *const T>(self.v0) }
    }

    pub fn get_mut(&self) -> &mut T {
        assert!(!self.is_null());
        unsafe { &mut *std::mem::transmute::<*mut T, *mut T>(self.v1) }
    }
}

unsafe impl<T> Send for QBox<T> {}
impl<T> Copy for QBox<T> {}

impl<T> Clone for QBox<T> {
    fn clone(&self) -> Self {
        Self {
            v0: self.v0.clone(),
            v1: self.v1.clone(),
        }
    }
}
