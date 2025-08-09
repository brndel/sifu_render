use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

use crate::{
    get_ident::GetIdent, parse::RawField, raw_rust_type::RawRustType, raw_value::{rust_scalar::RustScalar, wgsl_scalar::WgslScalar, wgsl_type::WgslType, ByteCount}, repeated_raw_field::RepeatedRawField, wgsl_type_str::WgslTypeStr
};

pub fn raw_struct_quote(
    ident: &Ident,
    raw_ident: &Ident,
    fields: &[RawField<WgslType<RustScalar>>],
) -> TokenStream {
    
    let raw_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        let raw_type = &field.raw_ty.raw_rust_type();
        quote! {
            #ident: #raw_type
        }
    });

    let into_fields = fields.iter().map(|field| {
        let span = field.source_ty.span();

        let ident = &field.ident;

        quote_spanned!(span => #ident: value.#ident.into())
    });

    quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, sifu_render::bytemuck::Zeroable, sifu_render::bytemuck::Pod)]
        #[bytemuck(crate = "sifu_render::bytemuck")]
        pub struct #raw_ident {
            #(#raw_fields,)*
        }


        impl From<#ident> for #raw_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #(#into_fields,)*
                }
            }
        }
    }
}

pub fn vertex_attributes_quote<'a>(
    fields: impl Iterator<Item = RepeatedRawField<'a, RustScalar>>,
    initial_offset_ident: Option<&TokenStream>,
) -> impl Iterator<Item = TokenStream> {
    fields
        .scan(0, |offset, field| {
            let pre_offset = *offset;
            *offset += field.ty.byte_count();

            Some((pre_offset as u64, field))
        })
        .enumerate()
        .map(move |(idx, (offset, field))| {
            let idx = idx as u32;
            let format = field.ty.vertex_format(field.source_ident.span());

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
    fields: impl Iterator<Item = RepeatedRawField<'a, RustScalar>> + Clone,
    initial_offset_ident: Option<&TokenStream>,
) -> syn::Result<TokenStream> {
    let format = shader_struct_format_str(name, fields.clone()).map_err(|err| {
        syn::Error::new(Span::call_site(), format!("internal format error {}", err))
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

pub fn shader_struct_format_str<'a, T: Into<WgslScalar> + 'a>(
    name: &str,
    fields: impl Iterator<Item = RepeatedRawField<'a, T>>,
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
            field.ty.scalar_into::<WgslScalar>().wgsl_type_str()
        )?;
    }

    write!(&mut out, "}}}}")?;

    Ok(out)
}


pub fn uniform_shader_struct_str<'a>(name: &str, fields: impl Iterator<Item = &'a RawField<WgslType<WgslScalar>>>) -> Result<String, std::fmt::Error> {
    use std::fmt::Write as _;
    let mut out = String::new();


    writeln!(&mut out, "struct {} {{", name)?;

    for field in fields {
        writeln!(&mut out, "  {}: {},", &field.ident, field.raw_ty.wgsl_type_str())?;
    }

    writeln!(&mut out, "}}")?;

    Ok(out)
}