use std::fmt::Display;

use syn::{Attribute, DeriveInput, Ident, Type, parse::Parse};

use crate::raw_field::RawField;

pub fn parse_attr<T: Parse>(name: &str, attrs: &[Attribute]) -> Option<syn::Result<T>> {
    for attr in attrs {
        if let Some(ident) = attr.path().get_ident() {
            if ident == name {
                return Some(attr.parse_args());
            }
        }
    }

    None
}

pub struct ParsedStruct {
    pub ident: Ident,
    pub fields: Vec<RawField>,
    pub vertex_attr: Option<Type>,
    pub name: &'static str,
}

impl ParsedStruct {
    pub fn raw_ident(&self) -> Ident {
        Ident::new(&format!("{}Raw{}", self.ident, self.name), self.ident.span())
    }

    pub fn parse(input: DeriveInput, name: &'static str) -> syn::Result<ParsedStruct> {
        let ident = input.ident;

        let vertex_attr = match parse_attr("vertex", &input.attrs) {
            Some(value) => Some(value?),
            None => None,
        };

        let syn::Data::Struct(input) = input.data else {
            return Err(syn::Error::new(
                ident.span(),
                format!("{} only works on structs", name),
            ));
        };

        if input.fields.iter().any(|field| field.ident.is_none()) {
            return Err(syn::Error::new(
                ident.span(),
                format!("{} only works on structs with named fields", name),
            ));
        }

        let fields = input
            .fields
            .into_iter()
            .map(|field| RawField::new(field))
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(ParsedStruct {
            ident,
            fields,
            vertex_attr,
            name,
        })
    }
}
