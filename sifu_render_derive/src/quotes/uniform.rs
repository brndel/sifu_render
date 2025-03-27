use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    raw_field::{RawField, RepeatedRawField},
    util::ParsedStruct,
};

use super::shared::{raw_struct_quote, raw_struct_quote_padded, shader_struct_format_quote, vertex_attributes_quote};

pub fn uniform_quote(input: ParsedStruct) -> syn::Result<TokenStream> {
    let raw_ident = input.raw_ident();

    let raw_struct = raw_struct_quote_padded(&input.ident, &raw_ident, &input.fields);
    let impl_uniform = impl_uniform_quote(&input.ident, &raw_ident, &input.fields)?;

    let result = quote! {
        #raw_struct

        #impl_uniform
    };

    Ok(result)
}

fn impl_uniform_quote(
    ident: &Ident,
    raw_ident: &Ident,
    fields: &[RawField],
) -> syn::Result<TokenStream> {
    let repeated_fields = RepeatedRawField::iter(fields.iter());

    let shader_struct_format =
        shader_struct_format_quote(&ident.to_string(), repeated_fields.clone(), None)?;

    // let repeated_field_count = repeated_fields.clone().count() as u32;

    // let vertex_attributes = vertex_attributes_quote(repeated_fields, None);

    Ok(quote! {
        impl sifu_render::Uniform for #ident {
            type Raw = #raw_ident;

            fn shader_struct_str() -> &'static str {
                static SHADER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| #shader_struct_format);

                &SHADER
            }
        }
    })
}
