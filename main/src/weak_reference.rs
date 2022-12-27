use napi::{
    bindgen_prelude::{
        Reference,
        ToNapiValue,
    },
    Env,
    Error,
    Result,
    Status,
};

pub struct WeakReference<T>(napi::bindgen_prelude::WeakReference<T>)
where
    T: 'static;

impl<T> Clone for WeakReference<T> {
    fn clone(&self) -> Self {
        WeakReference(self.0.clone())
    }
}

impl<T> ToNapiValue for WeakReference<T>
where
    T: 'static,
{
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        val: Self,
    ) -> Result<napi::sys::napi_value> {
        napi::bindgen_prelude::WeakReference::<T>::to_napi_value(env, val.0)
    }
}

impl<T> From<napi::bindgen_prelude::WeakReference<T>> for WeakReference<T>
where
    T: 'static,
{
    fn from(val: napi::bindgen_prelude::WeakReference<T>) -> Self {
        WeakReference(val)
    }
}

impl<T> Into<napi::bindgen_prelude::WeakReference<T>> for WeakReference<T>
where
    T: 'static,
{
    fn into(self) -> napi::bindgen_prelude::WeakReference<T> {
        self.0
    }
}

impl<T> WeakReference<T>
where
    T: 'static,
{
    pub fn upgrade(
        &self,
        env: Env,
    ) -> Result<Reference<T>> {
        self.0.upgrade(env)?.ok_or_else(|| {
            Error::new(
                Status::GenericFailure,
                "Failed to upgrade weak reference".to_string(),
            )
        })
    }
}
