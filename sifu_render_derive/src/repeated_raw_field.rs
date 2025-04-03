use syn::Ident;

use crate::{
    get_ident::GetIdent, parse::RawField, raw_rust_type::RawRustType, raw_value::{
        vertex_type::VertexType, wgsl_type::WgslType, ByteAlign, ByteCount
    }
};

pub struct RepeatedRawField<'a, S> {
    pub source_ident: &'a Ident,
    pub source_ty: &'a WgslType<S>,
    pub ty: VertexType<S>,
    pub repeat: Option<usize>,
}

impl<'a, S: 'a + Clone + Copy> RepeatedRawField<'a, S> {
    pub fn iter(
        fields: impl Iterator<Item = &'a RawField<WgslType<S>>> + Clone,
    ) -> impl Iterator<Item = Self> + Clone {
        fields.flat_map(|field| {
            field.raw_ty.to_vertex_type().map(|(ty, repeat)| Self {
                source_ident: &field.ident,
                source_ty: &field.raw_ty,
                ty,
                repeat,
            })
        })
    }
}

impl<'a, S> GetIdent for RepeatedRawField<'a, S> {
    fn ident(&self) -> Ident {
        match self.repeat {
            Some(repeat) => Ident::new(
                &format!("{}{}", self.source_ident, repeat),
                self.source_ident.span(),
            ),
            None => self.source_ident.clone(),
        }
    }
}

impl<'a, S: ByteCount> ByteCount for RepeatedRawField<'a, S> {
    fn byte_count(&self) -> usize {
        self.ty.byte_count()
    }
}

impl<'a, S: ByteCount> ByteAlign for RepeatedRawField<'a, S> {
    fn byte_align(&self) -> usize {
        self.ty.byte_align()
    }
}

impl<'a, S: RawRustType> RawRustType for RepeatedRawField<'a, S> {
    fn raw_rust_type(&self) -> proc_macro2::TokenStream {
        self.ty.raw_rust_type()
    }
}
