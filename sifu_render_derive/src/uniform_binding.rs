use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;



pub fn impl_uniform_binding_quote(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    
    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => return syn::Error::new(ident.span(), "UniformBinding only works on structs").into_compile_error()
    };

    if data_struct.fields.iter().any(|field| field.ident.is_none()) {
        return syn::Error::new(ident.span(), "UniformBinding does only work for named fields").into_compile_error()
    }

    let generics = &input.generics;

    let fields = data_struct.fields.iter().enumerate().map(|(idx, field)| {
        (idx, field.ident.as_ref().unwrap(), &field.ty)
    });

    let layout_fields = fields.clone().map(|(idx, _ident, ty)| {
        quote! {
            sifu_render::uniform_binding::wgpu::BindGroupLayoutEntry {
                binding: #idx as u32,
                visibility: sifu_render::uniform_binding::wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: <#ty as sifu_render::uniform_binding::AsBindingResource>::LAYOUT,
                count: None,
            }
        }
    });

    let binding_fields = fields.clone().map(|(idx, ident, ty)| {
        quote! {
            sifu_render::uniform_binding::wgpu::BindGroupEntry {
                binding: #idx as u32,
                resource: <#ty as sifu_render::uniform_binding::AsBindingResource>::bind_resource(&self.#ident),
            }
        }
    });

    let glsl_var_fields = fields.clone().map(|(idx, ident, ty)| {
        let name = ident.to_string();
        quote! {
            sifu_render::uniform_binding::GlslUniformVar {
                group_id,
                binding_id: #idx as u32,
                name: #name,
                uniform: <#ty as sifu_render::uniform_binding::AsBindingResource>::glsl_type(),
            }
        }
    });

    quote! {
        impl #generics sifu_render::uniform_binding::UniformBinding for #ident #generics {
            const LAYOUT: &'static [sifu_render::uniform_binding::wgpu::BindGroupLayoutEntry] = &[
                #(#layout_fields),*
            ];

            fn binding_entries(&self) -> Vec<sifu_render::uniform_binding::wgpu::BindGroupEntry> {
                vec![
                    #(#binding_fields),*
                ]
            }

            fn glsl_vars(group_id: u32) -> Vec<sifu_render::uniform_binding::GlslUniformVar> {
                vec![
                    #(#glsl_var_fields),*
                ]
            }

                fn bind_group_layout(device: &sifu_render::uniform_binding::wgpu::Device) -> &'static sifu_render::uniform_binding::wgpu::BindGroupLayout {
                    static LAYOUT: std::sync::OnceLock<sifu_render::uniform_binding::wgpu::BindGroupLayout> = std::sync::OnceLock::new();
                    
                    LAYOUT.get_or_init(|| {
                        device.create_bind_group_layout(&sifu_render::uniform_binding::wgpu::BindGroupLayoutDescriptor {
                                label: Some(std::any::type_name::<Self>()),
                                entries: <Self as sifu_render::uniform_binding::UniformBinding>::LAYOUT,
                            })
                    })
                }
        }
    }
}