use quote::quote;
use syn::spanned::Spanned;

use crate::{raw_rust_type::RawRustType, wgsl_type_str::WgslTypeStr};

use super::{ByteCount, rust_scalar::RustScalar};

#[derive(Debug, Clone, Copy)]
pub enum WgslScalar {
    I32,
    U32,
    F32,
}

impl ByteCount for WgslScalar {
    fn byte_count(&self) -> usize {
        match self {
            WgslScalar::I32 => 4,
            WgslScalar::U32 => 4,
            WgslScalar::F32 => 4,
        }
    }
}

impl From<RustScalar> for WgslScalar {
    fn from(value: RustScalar) -> Self {
        match value {
            RustScalar::I8 | RustScalar::I16 | RustScalar::I32 => Self::I32,
            RustScalar::U8 | RustScalar::U16 | RustScalar::U32 => Self::U32,
            RustScalar::F32 => Self::F32,
        }
    }
}

impl WgslTypeStr for WgslScalar {
    fn wgsl_type_str(&self) -> std::borrow::Cow<'static, str> {
        match self {
            WgslScalar::I32 => "i32",
            WgslScalar::U32 => "u32",
            WgslScalar::F32 => "f32",
        }
        .into()
    }
}

impl RawRustType for WgslScalar {
    fn raw_rust_type(&self) -> proc_macro2::TokenStream {
        match self {
            WgslScalar::I32 => quote! {i32},
            WgslScalar::U32 => quote! {u32},
            WgslScalar::F32 => quote! {f32},
        }
    }
}

impl TryFrom<&str> for WgslScalar {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i32" => Ok(Self::I32),
            "u32" => Ok(Self::U32),
            "f32" => Ok(Self::F32),
            _ => Err(()),
        }
    }
}

pub struct NotAUniformScalar(syn::Type);

impl From<NotAUniformScalar> for syn::Error {
    fn from(NotAUniformScalar(value): NotAUniformScalar) -> Self {
        Self::new(
            value.span(),
            format!(
                "`{}` is not a valid uniform scalar type
                
allowed types are
- `u32`
- `i32`
- `f32`
                ",
                value.span().source_text().unwrap_or_default()
            ),
        )
    }
}
impl TryFrom<syn::Type> for WgslScalar {
    type Error = NotAUniformScalar;

    fn try_from(value: syn::Type) -> Result<Self, Self::Error> {
        match &value {
            syn::Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident().map(ToString::to_string) {
                    if let Ok(value) = ident.as_str().try_into() {
                        return Ok(value);
                    }
                }
            }
            _ => (),
        }

        return Err(NotAUniformScalar(value));
    }
}
