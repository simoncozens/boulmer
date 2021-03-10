
mod safe_wrapper {
    use pyo3::Python;
    use std::sync::atomic::{AtomicPtr, Ordering};

    pub struct SafeWrapper<T>(*mut AtomicPtr<T>);

    unsafe impl<T> Send for SafeWrapper<T> {}

    impl<T> Drop for SafeWrapper<T> {
        fn drop(&mut self) {
            unsafe {
                let box_ptr = self.0;
                let ptr = &*(box_ptr as *const AtomicPtr<String>);
                let old = ptr.load(Ordering::Acquire);
                ptr.store(std::ptr::null_mut(), Ordering::Release);
                if old.is_null() {
                    std::mem::drop(Box::from_raw(box_ptr));
                }
            }
        }
    }

    impl<T> SafeWrapper<T> {
        pub fn scoped<'p, U>(
            _py: Python<'p>,
            obj: &mut T,
            f: impl FnOnce(SafeWrapper<T>) -> U,
        ) -> U {
            let box_ptr = Box::into_raw(Box::new(AtomicPtr::new(obj)));
            let wrapper = SafeWrapper(box_ptr);
            let result = f(wrapper);
            std::mem::drop(SafeWrapper(box_ptr));
            result
        }

        pub fn try_get_mut<'p>(&mut self, _py: Python<'p>) -> Option<&mut T> {
            unsafe {
                let ptr = (*self.0).load(Ordering::Relaxed);
                if ptr.is_null() {
                    None
                } else {
                    Some(&mut *ptr)
                }
            }
        }
    }
}
