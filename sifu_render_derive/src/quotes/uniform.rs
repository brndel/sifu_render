use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::Ident;

use crate::{
    get_ident::GetIdent,
    parse::{ParsedStruct, RawField},
    raw_rust_type::RawRustType,
    raw_value::{ByteAlign, ByteCount, wgsl_scalar::WgslScalar, wgsl_type::WgslType},
    repeated_raw_field::RepeatedRawField,
};

use super::shared::uniform_shader_struct_str;

pub fn uniform_quote(input: ParsedStruct<WgslScalar>) -> syn::Result<TokenStream> {
    let layout = RepeatedRawField::iter(input.fields.iter()).collect::<UniformLayout<_>>();

    let raw_ident = input.raw_ident();

    let raw_struct = raw_uniform_struct_quote(&input.ident, &raw_ident, &layout);
    let impl_uniform = impl_uniform_quote(&input.ident, &raw_ident, input.fields.iter())?;

    let result = quote! {
        #raw_struct

        #impl_uniform
    };

    Ok(result)
}

fn impl_uniform_quote<'a>(
    ident: &Ident,
    raw_ident: &Ident,
    fields: impl Iterator<Item = &'a RawField<WgslType<WgslScalar>>>,
) -> syn::Result<TokenStream> {
    let shader_struct_name = &ident.to_string();

    let shader_struct = match uniform_shader_struct_str(&shader_struct_name, fields) {
        Ok(value) => value,
        Err(err) => {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("internal format error {}", err),
            ));
        }
    };

    Ok(quote! {
        impl sifu_render::Uniform for #ident {
            type Raw = #raw_ident;

            fn shader_struct_str() -> &'static str {
                #shader_struct
            }

            fn shader_struct_name() -> &'static str {
                #shader_struct_name
            }
        }
    })
}

fn raw_uniform_struct_quote(
    ident: &Ident,
    raw_ident: &Ident,
    layout: &UniformLayout<RepeatedRawField<WgslScalar>>,
) -> TokenStream {
    let raw_fields = layout.fields.iter().map(|field| {
        let ident = field.ident();

        let rust_ty = field.raw_rust_type();

        quote! {
            #ident: #rust_ty
        }
    });

    let repeat_store_fields = layout.fields.iter().filter_map(|field| match field {
        UniformField::Field(field) => {
            if field.repeat == Some(0) {
                let source_ident = field.source_ident;
                let source_ty = field.source_ty.raw_rust_type();
                Some(quote_spanned! { source_ident.span() =>
                    let #source_ident: #source_ty = value.#source_ident.into();
                })
            } else {
                None
            }
        }
        _ => None,
    });

    let into_fields = layout.fields.iter().map(|field| {
        let ident = field.ident();

        let value = match field {
            UniformField::Field(field) => {
                let source_ident = field.source_ident;

                match field.repeat {
                    Some(idx) => quote! {
                        #source_ident[#idx]
                    },
                    None => quote! {
                        value.#source_ident.into()
                    },
                }
            }
            UniformField::Padding {
                index: _,
                byte_count,
            } => {
                quote! {[0; #byte_count]}
            }
        };

        quote! {
            #ident: #value
        }
    });

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, sifu_render::bytemuck::Zeroable, sifu_render::bytemuck::Pod)]
        #[bytemuck(crate = "sifu_render::bytemuck")]
        pub struct #raw_ident {
            #(#raw_fields),*
        }

        impl From<#ident> for #raw_ident {
            fn from(value: #ident) -> Self {

                #(#repeat_store_fields);*

                Self {
                    #(#into_fields),*
                }
            }
        }
    }
}

enum UniformField<T> {
    Field(T),
    Padding { index: usize, byte_count: usize },
}

impl<T: RawRustType> RawRustType for UniformField<T> {
    fn raw_rust_type(&self) -> TokenStream {
        match self {
            UniformField::Field(field) => field.raw_rust_type(),
            UniformField::Padding {
                index: _,
                byte_count,
            } => {
                quote! {[u8; #byte_count]}
            }
        }
    }
}

impl<T: GetIdent> GetIdent for UniformField<T> {
    fn ident(&self) -> Ident {
        match self {
            UniformField::Field(field) => field.ident(),
            UniformField::Padding {
                index,
                byte_count: _,
            } => Ident::new(&format!("padding{}", index), Span::call_site()),
        }
    }
}

struct UniformLayout<T> {
    fields: Vec<UniformField<T>>,
    offset: usize,
    padding_index: usize,
}

impl<T> UniformLayout<T> {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            offset: 0,
            padding_index: 0,
        }
    }
}

impl<T: ByteCount + ByteAlign> UniformLayout<T> {
    pub fn push(&mut self, field: T) {
        let byte_count = field.byte_count();
        let byte_align = field.byte_align();

        self.align_to(byte_align);

        self.fields.push(UniformField::Field(field));
        self.offset += byte_count;
    }

    pub fn align_to(&mut self, align: usize) {
        let required_padding = (align - (self.offset % align)) % align;

        if required_padding != 0 {
            self.fields.push(UniformField::Padding {
                index: self.padding_index,
                byte_count: required_padding,
            });
            self.padding_index += 1;
            self.offset += required_padding;
        }
    }
}

impl<T: ByteCount + ByteAlign> FromIterator<T> for UniformLayout<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();

        let mut this = Self::new();

        for field in iter {
            this.push(field);
        }

        this.align_to(16);

        this
    }
}
