use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use raw_value_count::RawValueCount;
use raw_value_type::RawValueType;
use syn::{Ident, LitInt, Type};

mod raw_value_count;
mod raw_value_type;

#[derive(Debug, Clone, Copy)]
pub struct RawValue {
    pub ty: RawValueType,
    pub count: RawValueCount,
    pub multiplier: Option<RawValueCount>,
}

impl RawValue {
    pub fn new(ty: Type, count: LitInt, multiplier: Option<LitInt>) -> syn::Result<Self> {
        let count = count.try_into()?;
        let multiplier = match multiplier {
            Some(value) => Some(value.try_into()?),
            None => None,
        };

        let ty = ty.try_into()?;

        Ok(Self {
            ty,
            count,
            multiplier,
        })
    }
}

impl RawValue {
    pub fn byte_count(&self) -> usize {
        self.ty.byte_count()
            * usize::from(self.count)
            * usize::from(self.multiplier.unwrap_or_default())
    }

    pub fn vertex_format(&self) -> TokenStream {
        let ident = Ident::new(
            &format!(
                "{}{}",
                self.ty.vertex_format_part(),
                self.count.vertex_format_part()
            ),
            Span::call_site(),
        );

        quote! {
            sifu_render::mesh::wgpu::VertexFormat::#ident
        }
    }

    pub fn glsl_type(&self) -> String {
        match self.count.glsl_type() {
            Some(count_ty) => {
                format!("{}<{}>", count_ty, self.ty.glsl_type())
            }
            None => format!("{}", self.ty.glsl_type()),
        }
    }

    pub fn rust_real_type(&self) -> TokenStream {
        let ty = Ident::new(&self.ty.rust_type(), Span::call_site());
        let count = usize::from(self.count);
        match self.multiplier {
            Some(multiplier) => {
                let multiplier = usize::from(multiplier);

                quote! {
                    [[#ty; #count]; #multiplier]
                }
            }
            None => {
                quote! {
                    [#ty; #count]
                }
            }
        }
    }

    pub fn uniform_compatible_type(&self) -> TokenStream {
        let ty = Ident::new(&self.ty.glsl_type(), Span::call_site());
        let count = usize::from(self.count);
        match self.multiplier {
            Some(multiplier) => {
                let multiplier = usize::from(multiplier);

                quote! {
                    [[#ty; #count]; #multiplier]
                }
            }
            None => {
                quote! {
                    [#ty; #count]
                }
            }
        }
    }
}