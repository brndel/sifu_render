use std::iter::repeat;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Ident, Token, parse::Parse, spanned::Spanned};

use crate::{raw_value::RawValue, util::parse_attr};

pub struct RawField {
    pub ident: syn::Ident,
    pub raw_value: RawValue,
}

impl RawField {
    pub fn new(field: syn::Field) -> syn::Result<Self> {
        let ident = match field.ident {
            Some(value) => value,
            None => {
                return Err(syn::Error::new(
                    field.span(),
                    "unnamed fields are not allowed",
                ));
            }
        };

        let raw_type: RawType = match parse_attr("raw", &field.attrs) {
            Some(Ok(value)) => value,
            Some(Err(err)) => return Err(err),
            None => {
                return Err(syn::Error::new(
                    ident.span(),
                    "every field needs a #[raw(…)] attribute",
                ));
            }
        };

        Ok(Self {
            ident,
            raw_value: RawValue::new(
                raw_type.ty,
                raw_type.count,
                raw_type.multiplier.map(|mul| mul.multiplier),
            )?,
        })
    }
}

struct RawType {
    ty: syn::Type,
    _colon: Token![;],
    count: syn::LitInt,
    multiplier: Option<RawTypeMultiplier>,
}

struct RawTypeMultiplier {
    _colon: Token![;],
    multiplier: syn::LitInt,
}

impl Parse for RawType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            _colon: input.parse()?,
            count: input.parse()?,
            multiplier: input.parse().ok(),
        })
    }
}

impl Parse for RawTypeMultiplier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _colon: input.parse()?,
            multiplier: input.parse()?,
        })
    }
}

pub struct RepeatedRawField<'a> {
    pub field: &'a RawField,
    pub repeat: Option<usize>,
}

impl<'a> RepeatedRawField<'a> {
    pub fn iter(
        fields: impl Iterator<Item = &'a RawField> + Clone,
    ) -> impl Iterator<Item = Self> + Clone {
        fields.flat_map(|field| {
            let (repeated_fields, is_repeat) = match field.raw_value.multiplier {
                Some(repeats) => {
                    let repeats = usize::from(repeats);
                    (repeat(field).take(repeats), true)
                }
                None => (repeat(field).take(1), false),
            };

            repeated_fields
                .enumerate()
                .map(move |(idx, field)| RepeatedRawField {
                    field,
                    repeat: is_repeat.then_some(idx),
                })
        })
    }

    pub fn ident(&self) -> String {
        match self.repeat {
            Some(repeat) => format!("{}{}", self.field.ident, repeat),
            None => self.field.ident.to_string(),
        }
    }
}

#[derive(Clone)]
pub enum PaddedRawField<'a> {
    Field(&'a RawField),
    Padding { index: usize },
}

impl<'a> PaddedRawField<'a> {
    pub fn iter(
        fields: impl Iterator<Item = &'a RawField> + Clone,
    ) -> impl Iterator<Item = Self> + Clone {
        fields
            .scan(0, |pad_count, field| {
                let count = usize::from(field.raw_value.count);
                if count == 3 {
                    let index = *pad_count;
                    *pad_count += 1;
                    Some([
                        Some(PaddedRawField::Field(field)),
                        Some(PaddedRawField::Padding { index }),
                    ])
                } else {
                    Some([Some(PaddedRawField::Field(field)), None])
                }
            })
            .flatten()
            .flatten()
    }

    pub fn ident(&self) -> Ident {
        match self {
            PaddedRawField::Field(raw_field) => raw_field.ident.clone(),
            PaddedRawField::Padding { index } => Ident::new(&format!("pad{}", index), Span::call_site()),
        }
    }

    pub fn ty(&self) -> TokenStream {
        match self {
            PaddedRawField::Field(raw_field) => raw_field.raw_value.uniform_compatible_type(),
            PaddedRawField::Padding { index: _ } => {
                Ident::new("u32", Span::call_site()).into_token_stream()
            }
        }
    }
}
