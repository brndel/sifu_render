use syn::{Attribute, DeriveInput, Ident, Token, Type, parse::Parse, spanned::Spanned};

use crate::{get_ident::GetIdent, raw_value::wgsl_type::WgslType, util::CollectSynErrors};

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

pub struct ParsedStruct<T> {
    pub ident: Ident,
    pub fields: Vec<RawField<WgslType<T>>>,
    pub vertex_attr: Option<Type>,
    /// The name of the macro this ParsedStruct gets created for.
    /// `name` is used for error messages and for names of raw structs
    pub name: &'static str,
}

impl<T> ParsedStruct<T> {
    pub fn raw_ident(&self) -> Ident {
        Ident::new(
            &format!("{}Raw{}", self.ident, self.name),
            self.ident.span(),
        )
    }

    pub fn parse(input: DeriveInput, name: &'static str) -> syn::Result<Self>
    where
        T: TryFrom<syn::Type>,
        syn::Error: From<<T as TryFrom<syn::Type>>::Error>,
    {
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
            .collect_syn_errors()?;

        Ok(ParsedStruct {
            ident,
            fields,
            vertex_attr,
            name,
        })
    }
}

pub struct RawField<T> {
    pub ident: syn::Ident,
    pub source_ty: syn::Type,
    pub raw_ty: T,
}

impl<T> RawField<WgslType<T>>
where
    T: TryFrom<syn::Type>,
    syn::Error: From<<T as TryFrom<syn::Type>>::Error>,
{
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

        let raw_ty: ParsedFieldTy = match parse_attr("raw", &field.attrs) {
            Some(Ok(value)) => value,
            Some(Err(err)) => return Err(err),
            None => ParsedFieldTy {
                ty: field.ty.clone(),
                dim_x: None,
                dim_y: None,
            },
        };

        // for some weird reason i cant use ? operator here
        let raw_ty: WgslType<T> = match raw_ty.try_into() {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        Ok(Self {
            ident,
            source_ty: field.ty,
            raw_ty,
        })
    }
}

impl<T> GetIdent for RawField<T> {
    fn ident(&self) -> Ident {
        self.ident.clone()
    }
}

impl<T> TryFrom<ParsedFieldTy> for WgslType<T>
where
    T: TryFrom<syn::Type>,
    syn::Error: From<<T as TryFrom<syn::Type>>::Error>,
{
    type Error = syn::Error;

    fn try_from(value: ParsedFieldTy) -> Result<Self, Self::Error> {
        let dim_x = value.dim_x.map(|dim| dim.dim.base10_parse()).transpose()?;
        let dim_y = value.dim_y.map(|dim| dim.dim.base10_parse()).transpose()?;

        let scalar = value.ty.clone().try_into()?;

        Ok(WgslType::new(scalar, dim_x, dim_y))
    }
}

struct ParsedFieldTy {
    pub ty: syn::Type,
    pub dim_x: Option<ParsedFieldDim>,
    pub dim_y: Option<ParsedFieldDim>,
}

struct ParsedFieldDim {
    _colon: Token![;],
    pub dim: syn::LitInt,
}

impl Parse for ParsedFieldTy {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            dim_x: input.parse().ok(),
            dim_y: input.parse().ok(),
        })
    }
}

impl Parse for ParsedFieldDim {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _colon: input.parse()?,
            dim: input.parse()?,
        })
    }
}
