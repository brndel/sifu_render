use std::borrow::Cow;

use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::{raw_rust_type::RawRustType, wgsl_type_str::WgslTypeStr};

use super::{dim::Dim, rust_scalar::RustScalar, wgsl_type::WgslType, ByteAlign, ByteCount};

#[derive(Debug, Clone, Copy)]
pub enum VertexType<T> {
    Scalar(T),
    Vec { dim: Dim, scalar: T },
}

impl<T: ByteCount> ByteCount for VertexType<T> {
    fn byte_count(&self) -> usize {
        match self {
            VertexType::Scalar(scalar) => scalar.byte_count(),
            VertexType::Vec { dim, scalar } => scalar.byte_count() * usize::from(*dim),
        }
    }
}


impl<T: ByteCount> ByteAlign for VertexType<T> {
    fn byte_align(&self) -> usize {
        match self {
            VertexType::Scalar(scalar) => scalar.byte_count(),
            VertexType::Vec { dim, scalar } => {
                let factor = match dim {
                    Dim::D2 => 2,
                    Dim::D3 => 4,
                    Dim::D4 => 4,
                };

                scalar.byte_count() * factor
            },
        }
    }
}


impl<T> From<VertexType<T>> for WgslType<T> {
    fn from(value: VertexType<T>) -> Self {
        match value {
            VertexType::Scalar(v) => WgslType::Scalar(v),
            VertexType::Vec { dim, scalar } => WgslType::Vec { dim, scalar },
        }
    }
}

impl VertexType<RustScalar> {
    pub fn vertex_format(self, span: Span) -> proc_macro2::TokenStream {
        let ident: Cow<_> = match self {
            VertexType::Scalar(scalar) => scalar.vertex_format_name().into(),
            VertexType::Vec { dim, scalar } => {
                if dim == Dim::D3
                    && matches!(
                        scalar,
                        RustScalar::I8 | RustScalar::I16 | RustScalar::U8 | RustScalar::U16
                    )
                {
                    return syn::Error::new(span, "vec3 is not allowed for 8 and 16 bit values")
                        .into_compile_error();
                }

                format!("{}x{}", scalar.vertex_format_name(), dim).into()
            }
        };

        let ident = Ident::new(&ident, Span::call_site());

        quote! {
            sifu_render::mesh::wgpu::VertexFormat::#ident
        }
    }
}

impl<T: RawRustType> RawRustType for VertexType<T> {
    fn raw_rust_type(&self) -> proc_macro2::TokenStream {
        match self {
            VertexType::Scalar(scalar) => scalar.raw_rust_type(),
            VertexType::Vec { dim, scalar } => {
                let ty = scalar.raw_rust_type();
                let dim = usize::from(*dim);

                quote! {[#ty; #dim]}
            }
        }
    }
}

impl<T: WgslTypeStr> WgslTypeStr for VertexType<T> {
    fn wgsl_type_str(&self) -> std::borrow::Cow<'static, str> {
        match self {
            VertexType::Scalar(scalar) => scalar.wgsl_type_str(),
            VertexType::Vec { dim, scalar } => {
                format!("vec{}<{}>", dim, scalar.wgsl_type_str()).into()
            }
        }
    }
}

impl<T> VertexType<T> {
    pub fn scalar_into<U>(self) -> VertexType<U>
    where
        T: Into<U>,
    {
        match self {
            VertexType::Scalar(scalar) => VertexType::Scalar(scalar.into()),
            VertexType::Vec { dim, scalar } => VertexType::Vec {
                dim,
                scalar: scalar.into(),
            },
        }
    }
}
