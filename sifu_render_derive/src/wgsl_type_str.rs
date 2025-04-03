use std::borrow::Cow;



pub trait WgslTypeStr {
    fn wgsl_type_str(&self) -> Cow<'static, str>;
}