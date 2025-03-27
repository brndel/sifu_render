use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::raw_field::{PaddedRawField, RawField, RepeatedRawField};

pub fn raw_struct_quote(ident: &Ident, raw_ident: &Ident, fields: &[RawField]) -> TokenStream {
    let field_idents = fields.iter().map(|field| &field.ident);

    let raw_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let raw_type = &field.raw_value.rust_real_type();
        quote! {
            #ident: #raw_type
        }
    });

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        pub struct #raw_ident {
            #(#raw_fields,)*
        }


        impl From<#ident> for #raw_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #(#field_idents: value.#field_idents.into(),)*
                }
            }
        }
    }
}

pub fn raw_struct_quote_padded(ident: &Ident, raw_ident: &Ident, fields: &[RawField]) -> TokenStream {
    let padded_fields = PaddedRawField::iter(fields.iter());

    let raw_fields = padded_fields.clone().map(|field| {
        let ident = field.ident();
        let raw_type = field.ty();
        quote! {
            #ident: #raw_type
        }
    });

    let into_fields = padded_fields.map(|field| {
        let ident = field.ident();
        match &field {
            PaddedRawField::Field(_) => {
                quote! {
                    #ident: value.#ident.into()
                }
            },
            PaddedRawField::Padding { index: _ } => {
                quote! {
                    #ident: 0
                }
            },
        }
    });

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        pub struct #raw_ident {
            #(#raw_fields,)*
        }


        impl From<#ident> for #raw_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #(#into_fields),*
                }
            }
        }
    }
}

pub fn vertex_attributes_quote<'a>(fields: impl Iterator<Item = RepeatedRawField<'a>>, initial_offset_ident: Option<&TokenStream>) -> impl Iterator<Item = TokenStream> {
    fields
        .scan(0, |offset, field| {
            let pre_offset = *offset;
            *offset += field.field.raw_value.byte_count();

            Some((pre_offset as u64, field))
        })
        .enumerate()
        .map(move |(idx, (offset, field))| {
            let idx = idx as u32;
            let format = field.field.raw_value.vertex_format();

            let shader_location = match &initial_offset_ident {
                Some(offet_ident) => quote! {
                    #idx + #offet_ident
                },
                None => quote! { #idx },
            };

            quote! {
                sifu_render::mesh::wgpu::VertexAttribute {
                    format: #format,
                    offset: #offset,
                    shader_location: #shader_location
                }
            }
        })
}

pub fn shader_struct_format_quote<'a>(
    name: &str,
    fields: impl Iterator<Item = RepeatedRawField<'a>> + Clone,
    initial_offset_ident: Option<&TokenStream>,
) -> syn::Result<TokenStream> {
    let format = shader_struct_format_str(name, fields.clone()).map_err(|err| {
        syn::Error::new(Span::call_site(), format!("Internal format error {}", err))
    })?;

    let offsets = 0_u32..(fields.count() as u32);

    match initial_offset_ident {
        Some(initial_offset_ident) => Ok(quote! {
            format!(#format, #(#offsets + #initial_offset_ident),*)
        }),
        None => Ok(quote! {
            format!(#format, #(#offsets),*)
        }),
    }
}

pub fn shader_struct_format_str<'a>(
    name: &str,
    fields: impl Iterator<Item = RepeatedRawField<'a>>,
) -> Result<String, std::fmt::Error> {
    use std::fmt::Write as _;
    let mut out = String::new();

    write!(&mut out, "struct {}", name)?;

    writeln!(&mut out, " {{{{")?;

    for field in fields {
        writeln!(
            &mut out,
            "  @location({{}}) {}: {},",
            field.ident(),
            field.field.raw_value.glsl_type()
        )?;
    }

    write!(&mut out, "}}}}")?;

    Ok(out)
}
