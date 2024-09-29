#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod components;
mod config;
mod expand;
mod idents;
mod tokens;
mod utils;

#[proc_macro_derive(QuickImpl, attributes(quick_impl))]
pub fn derive_quick_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).into()
}
