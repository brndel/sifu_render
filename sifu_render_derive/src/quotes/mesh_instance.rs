use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Type};

use crate::{
    raw_field::{RawField, RepeatedRawField},
    util::ParsedStruct,
};

use super::shared::{raw_struct_quote, shader_struct_format_quote, vertex_attributes_quote};

pub fn instance_quote(input: ParsedStruct) -> syn::Result<TokenStream> {
    let raw_ident = input.raw_ident();

    let Some(vertex_ty) = input.vertex_attr else {
        return Err(syn::Error::new(Span::call_site(), "#[vertex(…)] attribute missing"))
    };

    let raw_struct = raw_struct_quote(&input.ident, &raw_ident, &input.fields);
    let impl_instance = impl_instance_quote(&input.ident, &raw_ident, &vertex_ty, &input.fields)?;

    let result = quote! {
        #raw_struct

        #impl_instance
    };

    Ok(result)
}

fn impl_instance_quote(
    ident: &Ident,
    raw_ident: &Ident,
    vertex_ty: &Type,
    fields: &[RawField],
) -> syn::Result<TokenStream> {
    let repeated_fields = RepeatedRawField::iter(fields.iter());

    let initial_offset_ident = quote! {
        <#vertex_ty as sifu_render::mesh::Vertex>::SHADER_LOCATION_COUNT
    };

    let shader_struct_format =
        shader_struct_format_quote(&ident.to_string(), repeated_fields.clone(), Some(&initial_offset_ident))?;

    let vertex_attributes = vertex_attributes_quote(repeated_fields, Some(&initial_offset_ident));

    Ok(quote! {
        impl sifu_render::mesh::MeshInstance for #ident {
            type Vertex = #vertex_ty;
            type Raw = #raw_ident;

            const LAYOUT: sifu_render::mesh::wgpu::VertexBufferLayout<'static> = sifu_render::mesh::wgpu::VertexBufferLayout {
                array_stride: core::mem::size_of::<#raw_ident>() as u64,
                step_mode: sifu_render::mesh::wgpu::VertexStepMode::Instance,
                attributes: &[
                    #(#vertex_attributes),*
                ],
            };


            fn shader_struct_str() -> &'static str {
                static SHADER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| #shader_struct_format);

                &SHADER
            }
        }
    })
}
