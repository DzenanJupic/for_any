#![feature(result_into_ok_or_err, result_flattening)]
#![feature(exact_size_is_empty)]

extern crate proc_macro;

use proc_macro::TokenStream;

use darling::FromDeriveInput;

use crate::for_any::ForAnyInput;

mod utils;
mod for_any;


#[test]
#[cfg(test)]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}

#[proc_macro_derive(ForAny, attributes(for_any))]
pub fn for_any(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    ForAnyInput::from_derive_input(&input)
        .map(ForAnyInput::generate_output)
        .map_err(|err| err.write_errors())
        .flatten()
        .into_ok_or_err()
        .into()
}
