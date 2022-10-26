use napi::{
  bindgen_prelude::{Object, Reference, ToNapiValue},
  Env, JsString, NapiValue, Property, Result,
};
pub trait WithDataInBrackets
where
  Self: Sized + 'static,
{
  fn raw_item(&self, index: usize) -> Option<String>;
  fn get_reference(&self) -> Result<Reference<Self>>;
  fn get_env(&self) -> Env;
  fn set_properties(&self) -> Result<()> {
    let env = self.get_env();
    let mut this = unsafe {
      let ptr = <Reference<Self> as ToNapiValue>::to_napi_value(
        env.raw(),
        self.get_reference()?,
      )?;
      Object::from_raw(env.raw(), ptr)?
    };

    let mut index: u32 = 0;
    loop {
      if let Some(ref s) = self.raw_item(index as usize) {
        let value: JsString = env.create_string(s)?;
        // this.set_element(index, value)?;
        let property = Property::new(&index.to_string())?
          .with_value(&value)
          .with_property_attributes(napi::PropertyAttributes::Configurable);
        this.define_properties(&[property])?;
      } else if this.has_element(index)? {
        this.delete_element(index)?;
      } else {
        break;
      }
      index += 1;
    }

    Ok(())
  }
}
