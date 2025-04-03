mod repeated_raw_field;
mod raw_value;
mod parse;
mod quotes;
mod util;
mod raw_rust_type;
mod wgsl_type_str;
mod get_ident;


use proc_macro::TokenStream;
use quotes::{mesh_instance::instance_quote, uniform::uniform_quote, vertex::vertex_quote};
use syn::{DeriveInput, parse_macro_input};
use parse::ParsedStruct;

#[proc_macro_derive(Vertex, attributes(raw))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    match parse_vertex(derive_input) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_vertex(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    let parsed = ParsedStruct::parse(derive_input, "Vertex")?;

    let result = vertex_quote(parsed)?;

    Ok(result.into())
}

#[proc_macro_derive(MeshInstance, attributes(vertex, raw))]
pub fn derive_instance(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    match parse_instance(derive_input) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_instance(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    let parsed = ParsedStruct::parse(derive_input, "MeshInstance")?;

    let result = instance_quote(parsed)?;

    Ok(result.into())
}


#[proc_macro_derive(Uniform, attributes(raw))]
pub fn derive_uniform(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    match parse_uniform(derive_input) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_uniform(derive_input: DeriveInput) -> syn::Result<TokenStream> {
    let parsed = ParsedStruct::parse(derive_input, "Uniform")?;

    let result = uniform_quote(parsed)?;

    Ok(result.into())
}