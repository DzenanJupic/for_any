use proc_macro2::Ident;
use syn::Type;

#[derive(darling::FromField)]
pub struct EnumField {
    pub ident: Option<Ident>,
    pub ty: Type,
}
