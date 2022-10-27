use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    rc::Rc,
};

use napi::{
    bindgen_prelude::{
        Reference,
        WeakReference,
    },
    Env,
    Error,
    Result,
};

struct MyLazyCell<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
}

impl<T> MyLazyCell<T> {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn init(
        &self,
        value: T,
    ) {
        unsafe {
            *self.inner.get() = MaybeUninit::new(value);
        }
    }

    pub fn borrow(&self) -> &T {
        unsafe { (*self.inner.get()).assume_init_ref() }
    }
}

pub struct CyclicReference<T>
where
    T: 'static,
{
    env: Env,
    inner: Rc<MyLazyCell<WeakReference<T>>>,
}

impl<T> CyclicReference<T> {
    pub fn new_cyclic(
        env: Env,
        init: impl FnOnce(CyclicReference<T>) -> Result<Reference<T>>,
    ) -> Result<Reference<T>> {
        let lazy = Rc::new(MyLazyCell::new());
        let me = Self {
            env,
            inner: lazy.clone(),
        };
        let reference = init(me)?;
        lazy.init(reference.downgrade());
        Ok(reference)
    }

    pub fn get(&self) -> Result<Reference<T>> {
        self.get_weak().upgrade(self.env)?.ok_or_else(|| {
            Error::from_reason(
                "self reference is not available anymore".to_string(),
            )
        })
    }

    pub fn get_weak(&self) -> WeakReference<T> {
        self.inner.borrow().clone()
    }
}
