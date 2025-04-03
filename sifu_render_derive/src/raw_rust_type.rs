use proc_macro2::TokenStream;



pub trait RawRustType {
    fn raw_rust_type(&self) -> TokenStream;
}
