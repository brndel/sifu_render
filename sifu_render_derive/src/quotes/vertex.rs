use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    parse::{ParsedStruct, RawField}, repeated_raw_field::RepeatedRawField, raw_value::{rust_scalar::RustScalar, wgsl_type::WgslType}
};

use super::shared::{raw_struct_quote, shader_struct_format_quote, vertex_attributes_quote};

pub fn vertex_quote(input: ParsedStruct<RustScalar>) -> syn::Result<TokenStream> {
    let raw_ident = input.raw_ident();

    let raw_struct = raw_struct_quote(&input.ident, &raw_ident, &input.fields);
    let impl_vertex = impl_vertex_quote(&input.ident, &raw_ident, &input.fields)?;

    let result = quote! {
        #raw_struct

        #impl_vertex
    };

    Ok(result)
}

fn impl_vertex_quote(
    ident: &Ident,
    raw_ident: &Ident,
    fields: &[RawField<WgslType<RustScalar>>],
) -> syn::Result<TokenStream> {
    let repeated_fields = RepeatedRawField::iter(fields.iter());

    let shader_struct_format =
        shader_struct_format_quote(&ident.to_string(), repeated_fields.clone(), None)?;

    let repeated_field_count = repeated_fields.clone().count() as u32;

    let vertex_attributes = vertex_attributes_quote(repeated_fields, None);

    Ok(quote! {
        impl sifu_render::mesh::Vertex for #ident {
            type Raw = #raw_ident;

            const LAYOUT: sifu_render::mesh::wgpu::VertexBufferLayout<'static> = sifu_render::mesh::wgpu::VertexBufferLayout {
                array_stride: core::mem::size_of::<#raw_ident>() as u64,
                step_mode: sifu_render::mesh::wgpu::VertexStepMode::Vertex,
                attributes: &[
                    #(#vertex_attributes),*
                ],
            };

            const SHADER_LOCATION_COUNT: u32 = #repeated_field_count;

            fn shader_struct_str() -> &'static str {
                static SHADER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| #shader_struct_format);

                &SHADER
            }
        }
    })
}
