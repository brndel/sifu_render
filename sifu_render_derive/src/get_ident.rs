use syn::Ident;



pub trait GetIdent {
    fn ident(&self) -> Ident;
}