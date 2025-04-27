use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, ItemEnum, LitStr, Variant};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    utils::to_snake_case,
};

const DEFAULT_NAME: &str = "is_{}";
const DEFAULT_DOC: &str =
    "Returns `true` if the variant is [`{}::{}`]; otherwise, returns `false`.";

pub fn expand_is(
    input: &ItemEnum,
    variant: &Variant,
    order: &OrderMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, order.ident.span()),
        [&to_snake_case(&variant.ident.to_string())],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let destruct = match &variant.fields {
        Fields::Named(_) => quote! { { .. } },
        Fields::Unnamed(_) => quote! { ( .. ) },
        Fields::Unit => TokenStream::new(),
    };

    let variant_ident = &variant.ident;
    let keywords = order.keywords();

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident(&self) -> bool {
            match self {
                Self::#variant_ident #destruct => true,
                _ => false,
            }
        }
    })
}
