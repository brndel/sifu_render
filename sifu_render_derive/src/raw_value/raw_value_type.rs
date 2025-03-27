use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Ident};

#[derive(Debug, Clone, Copy)]
pub enum RawValueType {
    UInt8,
    UInt16,
    UInt32,
    SInt8,
    SInt16,
    SInt32,
    Float32,
}

impl RawValueType {
    pub fn byte_count(&self) -> usize {
        match self {
            RawValueType::UInt8 => 1,
            RawValueType::UInt16 => 2,
            RawValueType::UInt32 => 4,
            RawValueType::SInt8 => 1,
            RawValueType::SInt16 => 2,
            RawValueType::SInt32 => 4,
            RawValueType::Float32 => 4,
        }
    }

    pub fn vertex_format_part(&self) -> &'static str {
        match self {
            RawValueType::UInt8 => "Uint8",
            RawValueType::UInt16 => "Uint16",
            RawValueType::UInt32 => "Uint32",
            RawValueType::SInt8 => "Sint8",
            RawValueType::SInt16 => "Sint16",
            RawValueType::SInt32 => "Sint32",
            RawValueType::Float32 => "Float32",
        }
    }

    pub fn glsl_type(&self) -> &'static str {
        match self {
            RawValueType::UInt8 | RawValueType::UInt16 | RawValueType::UInt32 => "u32",
            RawValueType::SInt8 | RawValueType::SInt16 | RawValueType::SInt32 => "i32",
            RawValueType::Float32 => "f32",
        }
    }

    pub fn rust_type(&self) -> &'static str {
        match self {
            RawValueType::UInt8 => "u8",
            RawValueType::UInt16 => "u16",
            RawValueType::UInt32 => "u32",
            RawValueType::SInt8 => "i8",
            RawValueType::SInt16 => "i16",
            RawValueType::SInt32 => "i32",
            RawValueType::Float32 => "f32",
        }
    }
}

pub struct InvalidType(Span);

impl From<InvalidType> for syn::Error {
    fn from(value: InvalidType) -> Self {
        Self::new(value.0, "invalid type; \nallowed are u8, u16, u32, i8, i16, i32, f32")
    }
}

impl TryFrom<syn::Type> for RawValueType {
    type Error = InvalidType;

    fn try_from(value: syn::Type) -> Result<Self, Self::Error> {
        match &value {
            syn::Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    if ident == "f32" {
                        return Ok(Self::Float32);
                    } else if ident == "u8" {
                        return Ok(Self::UInt8);
                    } else if ident == "u16" {
                        return Ok(Self::UInt16);
                    } else if ident == "u32" {
                        return Ok(Self::UInt32);
                    } else if ident == "i8" {
                        return Ok(Self::SInt8);
                    } else if ident == "i16" {
                        return Ok(Self::SInt16);
                    } else if ident == "i32" {
                        return Ok(Self::SInt32);
                    }
                }
            }
            _ => (),
        }

        Err(InvalidType(value.span()))
    }
}

impl ToTokens for RawValueType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new(self.rust_type(), Span::call_site()));
    }
}