use std::iter::repeat_n;

use quote::quote;

use crate::{raw_rust_type::RawRustType, wgsl_type_str::WgslTypeStr};

use super::{ByteCount, dim::Dim, vertex_type::VertexType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgslType<T> {
    Scalar(T),
    Vec { dim: Dim, scalar: T },
    Mat { dim_x: Dim, dim_y: Dim, scalar: T },
}

impl<T> WgslType<T> {
    pub fn new(scalar: T, dim_x: Option<Dim>, dim_y: Option<Dim>) -> Self {
        match (dim_x, dim_y) {
            (None, _) => Self::Scalar(scalar),
            (Some(dim), None) => Self::Vec { dim, scalar },
            (Some(dim_x), Some(dim_y)) => Self::Mat {
                dim_x,
                dim_y,
                scalar,
            },
        }
    }
}

impl<T: ByteCount> ByteCount for WgslType<T> {
    fn byte_count(&self) -> usize {
        match self {
            WgslType::Scalar(scalar) => scalar.byte_count(),
            WgslType::Vec { dim, scalar } => scalar.byte_count() * usize::from(*dim),
            WgslType::Mat {
                dim_x,
                dim_y,
                scalar,
            } => scalar.byte_count() * usize::from(*dim_x) * usize::from(*dim_y),
        }
    }
}

impl<T: Clone> WgslType<T> {
    pub fn to_vertex_type(self) -> impl Iterator<Item = (VertexType<T>, Option<usize>)> + Clone {
        let (vertex, count) = match self {
            WgslType::Scalar(scalar) => (VertexType::Scalar(scalar), None),
            WgslType::Vec { dim, scalar } => (VertexType::Vec { dim, scalar }, None),
            WgslType::Mat {
                dim_x,
                dim_y,
                scalar,
            } => (VertexType::Vec { dim: dim_x, scalar }, Some(dim_y.into())),
        };

        repeat_n(vertex, count.unwrap_or(1))
            .enumerate()
            .map(move |(i, x)| (x, count.is_some().then_some(i)))
    }
}

impl<T: RawRustType> RawRustType for WgslType<T> {
    fn raw_rust_type(&self) -> proc_macro2::TokenStream {
        match self {
            WgslType::Scalar(scalar) => scalar.raw_rust_type(),
            WgslType::Vec { dim, scalar } => {
                let ty = scalar.raw_rust_type();
                let dim = usize::from(*dim);

                quote! {[#ty; #dim]}
            }
            WgslType::Mat {
                dim_x,
                dim_y,
                scalar,
            } => {
                let ty = scalar.raw_rust_type();
                let dim_x = usize::from(*dim_x);
                let dim_y = usize::from(*dim_y);

                quote! {[[#ty; #dim_x]; #dim_y]}
            }
        }
    }
}

impl<T: WgslTypeStr> WgslTypeStr for WgslType<T> {
    fn wgsl_type_str(&self) -> std::borrow::Cow<'static, str> {
        match self {
            WgslType::Scalar(scalar) => scalar.wgsl_type_str(),
            WgslType::Vec { dim, scalar } => {
                format!("vec{}<{}>", dim, scalar.wgsl_type_str()).into()
            }
            WgslType::Mat {
                dim_x,
                dim_y,
                scalar,
            } => format!("mat{}x{}<{}>", dim_x, dim_y, scalar.wgsl_type_str()).into(),
        }
    }
}
