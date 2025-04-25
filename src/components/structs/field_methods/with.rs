use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{
    attr::{Attr, AttrMethod},
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::IndexedField,
};

const DEFAULT_NAME: &str = "with_{}";
const DEFAULT_DOC: &str = "Returns an instance of [`{0}`] with the `{1}` field modified.";

pub fn expand_with(
    input: &ItemStruct,
    indexed_field: &IndexedField,
    attribute: &Attr,
    method_attr: &AttrMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, attribute.ident.span()),
        [&indexed_field.as_token().to_string()],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [
            &input.ident.to_string(),
            &indexed_field.as_token().to_string(),
        ],
    )?;

    config.finish()?;

    let keywords = method_attr.keywords();
    let ty = &indexed_field.ty;
    let arg_ident = indexed_field.as_ident();
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident (mut self, #arg_ident: #ty) -> Self {
            self.#field_ident = #arg_ident;
            self
        }
    })
}
