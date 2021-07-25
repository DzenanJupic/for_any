use darling::ast::Style;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::Type;

use crate::for_any::enum_field::EnumField;

#[derive(darling::FromVariant)]
#[darling(attributes(for_any), forward_attrs(allow, doc, cfg))]
pub struct EnumVariant {
    pub ident: Ident,
    pub fields: darling::ast::Fields<EnumField>,

    #[darling(default)]
    pub skip: bool,
}

impl EnumVariant {
    pub fn as_field_types(&self) -> Vec<Type> {
        self.fields.fields
            .iter()
            .map(|field| field.ty.clone())
            .collect()
    }

    pub fn as_match_arm(&self) -> TokenStream2 {
        if self.skip { return TokenStream2::new(); }

        let ident = &self.ident;
        let field_names = Self::field_names(&self.fields);
        let deconstructor = Self::deconstructor(self.fields.style, field_names.clone());
        let args = quote! { #( #field_names ),* };

        quote! { #ident #deconstructor => func(#args), }
    }

    fn field_names(fields: &darling::ast::Fields<EnumField>) -> impl Iterator<Item=Ident> + Clone + '_ {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                match field.ident {
                    Some(ref ident) => ident.clone(),
                    None => quote::format_ident!("_{}", i, span = field.ty.span())
                }
            })
    }

    fn deconstructor(style: Style, field_names: impl Iterator<Item=Ident>) -> TokenStream2 {
        match style {
            Style::Tuple => quote! { ( #( #field_names ),* ) },
            Style::Struct => quote! { { #( #field_names ),* } },
            Style::Unit => quote! {},
        }
    }
}
