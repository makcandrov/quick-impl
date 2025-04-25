#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod attr;
mod components;
mod config;
mod ctx;
mod expand;
mod idents;
mod tokens;
mod utils;

#[proc_macro_attribute]
pub fn quick_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::expand(args.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
