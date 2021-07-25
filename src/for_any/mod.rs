use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{PredicateType, Token, Type, TypeParamBound, TypeTraitObject, WherePredicate};
use syn::punctuated::Punctuated;

use enum_variant::EnumVariant;

use crate::utils::Either;

mod enum_field;
mod enum_variant;

#[derive(darling::FromDeriveInput)]
#[darling(attributes(for_any), forward_attrs(allow, doc, cfg), supports(enum_any))]
pub struct ForAnyInput {
    ident: Ident,
    vis: syn::Visibility,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariant, ()>,

    #[darling(default)]
    for_any: Option<Either<bool, Ident>>,
    #[darling(default)]
    for_any_ref: Option<Either<bool, Ident>>,
    #[darling(default)]
    for_any_mut: Option<Either<bool, Ident>>,
    #[darling(default)]
    bounds: Vec<WherePredicate>,
    #[darling(default)]
    generic: Punctuated<TypeParamBound, Token![+]>,
}

impl ForAnyInput {
    pub fn generate_output(mut self) -> Result<TokenStream2, TokenStream2> {
        let (for_any, for_any_ref, for_any_mut) = self.generate_for_any_funcs()?;

        if let Some(where_clause) = self.generics.where_clause {
            self.bounds.extend(where_clause.predicates);
        }

        let enum_ident = self.ident;
        let generics = self.generics.params;
        let where_clause = self.bounds;

        Ok(quote! {
            impl<#generics> #enum_ident<#generics>
                where #( #where_clause ),* {

                #for_any
                #for_any_ref
                #for_any_mut
            }
        })
    }

    fn generate_for_any_funcs(&mut self) -> Result<(TokenStream2, TokenStream2, TokenStream2), TokenStream2> {
        let (variant_type_count, variant_types) = self.check_variants_type_count_and_add_bounds()?;

        let func_args = match self.generic.len() {
            0 => variant_types,
            _ => {
                (0..variant_type_count)
                    .map(|_| TypeTraitObject {
                        dyn_token: Some(Default::default()),
                        bounds: self.generic.clone(),
                    })
                    .map(Type::from)
                    .collect()
            }
        };

        let for_any = match self.generic.len() {
            0 => self.generate_for_any_func(
                "for_any",
                self.for_any.clone(),
                &func_args,
                quote! {},
            ),
            // the size of dyn Trait is not know at compile time
            _ => quote! {}
        };
        let for_any_ref = self.generate_for_any_func(
            "for_any_ref",
            self.for_any_ref.clone(),
            &func_args,
            quote! { & },
        );
        let for_any_mut = self.generate_for_any_func(
            "for_any_mut",
            self.for_any_mut.clone(),
            &func_args,
            quote! { &mut },
        );

        Ok((for_any, for_any_ref, for_any_mut))
    }

    fn generate_for_any_func(
        &self,
        default_ident: &'static str,
        attr: Option<Either<bool, Ident>>,
        func_args: &[Type],
        ref_ts: TokenStream2,
    ) -> TokenStream2 {
        let ident = match attr {
            Some(Either::A(false)) => return TokenStream2::new(),
            Some(Either::B(ident)) => ident,
            Some(Either::A(true)) | None => Ident::new(default_ident, Span::call_site())
        };
        let vis = &self.vis;
        let match_arms = self.match_arms();
        let enum_ident = &self.ident;

        let has_skipped_variants = self.has_skipped_variants();
        let return_mapper = has_skipped_variants.then(|| quote! { ::core::option::Option::Some });
        let wild_card_pattern = (has_skipped_variants || match_arms.is_empty())
            .then(|| quote! {
                // We can return Default::default(), since this pattern will only be included when
                // either Option or () is returned, which both implement Default.
                _ => return ::core::default::Default::default()
            });
        let return_type = if match_arms.is_empty() {
            quote! { () }
        } else if has_skipped_variants {
            quote! { ::core::option::Option<__FOR_ANY_FUNC_RET__> }
        } else {
            quote! { __FOR_ANY_FUNC_RET__ }
        };

        quote! {
            #[allow(non_camel_case_types, unused_variables, unused_imports, unused_parens, dead_code, unreachable_code)]
            #vis fn #ident<__FOR_ANY_FUNC_RET__, __FOR_ANY_FUNC__: FnOnce( #(#ref_ts #func_args),* ) -> __FOR_ANY_FUNC_RET__>(
                #ref_ts self,
                func: __FOR_ANY_FUNC__
            ) -> #return_type {
                use #enum_ident::*;

                let ret = match self {
                    #( #match_arms )*
                    #wild_card_pattern
                };
                #return_mapper(ret)
            }
        }
    }

    fn match_arms(&self) -> impl ExactSizeIterator<Item=TokenStream2> + '_ {
        self.data
            .as_ref()
            .take_enum()
            .expect("darling makes sure we only receive enums")
            .into_iter()
            .map(|variant| variant.as_match_arm())
    }

    fn check_variants_type_count_and_add_bounds(&mut self) -> Result<(usize, Vec<Type>), TokenStream2> {
        let variants = self.data
            .as_ref()
            .take_enum()
            .expect("darling makes sure we only receive enums");

        if variants.is_empty() { return Ok((0, Vec::new())); }

        let expected_fields = &variants[0].fields.fields;

        for variant in &variants {
            if variant.fields.fields.len() != expected_fields.len() {
                return Err(syn::Error::new(
                    variant.ident.span(),
                    "This variant has not the same amount of types as the other variants.\n\
                    If you want to ignore this variant, you can annotate it with #[for_any(skip)].",
                ).to_compile_error());
            }

            if !self.generic.is_empty() {
                for field in &variant.fields.fields {
                    let where_predicate = WherePredicate::Type(PredicateType {
                        lifetimes: None,
                        bounded_ty: field.ty.clone(),
                        colon_token: Default::default(),
                        bounds: self.generic.clone(),
                    });
                    self.bounds.push(where_predicate);
                }
            }
        }

        Ok((expected_fields.len(), variants[0].as_field_types()))
    }

    fn has_skipped_variants(&self) -> bool {
        self.data
            .as_ref()
            .take_enum()
            .expect("darling makes sure we only receive enums")
            .into_iter()
            .fold(false, |has_skipped_variants, variant| {
                has_skipped_variants || variant.skip
            })
    }
}
