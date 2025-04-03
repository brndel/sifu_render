use quote::quote;
use syn::spanned::Spanned;

use crate::raw_rust_type::RawRustType;

use super::{ByteCount, wgsl_scalar::WgslScalar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustScalar {
    I8,
    I16,
    I32,
    U8,
    U16,
    U32,
    F32,
}

impl ByteCount for RustScalar {
    fn byte_count(&self) -> usize {
        match self {
            RustScalar::I8 => 1,
            RustScalar::I16 => 2,
            RustScalar::I32 => 4,
            RustScalar::U8 => 1,
            RustScalar::U16 => 2,
            RustScalar::U32 => 4,
            RustScalar::F32 => 4,
        }
    }
}

impl From<WgslScalar> for RustScalar {
    fn from(value: WgslScalar) -> Self {
        match value {
            WgslScalar::I32 => RustScalar::I32,
            WgslScalar::U32 => RustScalar::U32,
            WgslScalar::F32 => RustScalar::F32,
        }
    }
}

impl AsRef<str> for RustScalar {
    fn as_ref(&self) -> &str {
        match self {
            RustScalar::I8 => "i8",
            RustScalar::I16 => "i16",
            RustScalar::I32 => "i32",
            RustScalar::U8 => "u8",
            RustScalar::U16 => "u16",
            RustScalar::U32 => "u32",
            RustScalar::F32 => "f32",
        }
    }
}

impl RawRustType for RustScalar {
    fn raw_rust_type(&self) -> proc_macro2::TokenStream {
        match self {
            RustScalar::I8 => quote! {i8},
            RustScalar::I16 => quote! {i16},
            RustScalar::I32 => quote! {i32},
            RustScalar::U8 => quote! {u8},
            RustScalar::U16 => quote! {u16},
            RustScalar::U32 => quote! {u32},
            RustScalar::F32 => quote! {f32},
        }
    }
}

impl RustScalar {
    pub fn vertex_format_name(self) -> &'static str {
        match self {
            RustScalar::I8 => "Sint8",
            RustScalar::I16 => "Sint16",
            RustScalar::I32 => "Sint32",
            RustScalar::U8 => "Uint8",
            RustScalar::U16 => "Uint16",
            RustScalar::U32 => "Uint32",
            RustScalar::F32 => "Float32",
        }
    }
}

impl TryFrom<&str> for RustScalar {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "f32" => Ok(Self::F32),
            _ => Err(()),
        }
    }
}

pub struct NotAVertexScalar(syn::Type);

impl From<NotAVertexScalar> for syn::Error {
    fn from(NotAVertexScalar(value): NotAVertexScalar) -> Self {
        Self::new(
            value.span(),
            format!(
                "`{}` is not a valid vertex scalar type
                
allowed types are
- `u8`, `u16`, `u32`
- `i8`, `i16`, `i32`
- `f32`
                ",
                value.span().source_text().unwrap_or_default()
            ),
        )
    }
}
impl TryFrom<syn::Type> for RustScalar {
    type Error = NotAVertexScalar;

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

        return Err(NotAVertexScalar(value));
    }
}
