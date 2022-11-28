use napi::{
    bindgen_prelude::FromNapiValue,
    Error,
    Result,
    Status,
};

pub enum InsertPosition {
    Before,
    Prepend,
    Append,
    After,
    Position(usize),
}

impl FromNapiValue for InsertPosition {
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> Result<Self> {
        let val = String::from_napi_value(env, napi_val)?;

        match val.as_str() {
            "beforebegin" => Ok(InsertPosition::Before),
            "afterbegin" => Ok(InsertPosition::Prepend),
            "beforeend" => Ok(InsertPosition::Append),
            "afterend" => Ok(InsertPosition::After),
            _ => Err(Error::new(
                Status::InvalidArg,
                "Invalid insert position".to_string(),
            )),
        }
    }
}

// impl TypeName for InsertPosition2 {
//     fn type_name() -> &'static str {
//         "InsertPosition"
//     }

//     fn value_type() -> ValueType {
//         ValueType::Unknown
//     }
// }
