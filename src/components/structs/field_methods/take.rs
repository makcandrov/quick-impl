use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    tokens::IndexedField,
};

const DEFAULT_NAME: &str = "take_{}";
const DEFAULT_DOC: &str =
    "Returns the `{1}` field of [`{0}`] and replaces it with its default value.";

pub fn expand_take(
    input: &ItemStruct,
    indexed_field: &IndexedField,
    order: &OrderMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, order.ident.span()),
        [&indexed_field.as_token().to_string()],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.as_token().to_string()],
    )?;

    config.finish()?;

    let keywords = order.keywords();
    let ty = &indexed_field.ty;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident (&mut self) -> #ty where #ty: ::core::default::Default {
            ::core::mem::take(&mut self.#field_ident)
        }
    })
}
