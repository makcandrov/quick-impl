use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::IndexedField,
};

const DEFAULT_NAME: &str = "get_{}";
const DEFAULT_DOC: &str = "A getter for the `{1}` field of [`{0}`].";

pub fn expand_get_clone(
    context: &Context,
    indexed_field: &IndexedField,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
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
            &context.ident.to_string(),
            &indexed_field.as_token().to_string(),
        ],
    )?;

    config.finish()?;

    let keywords = method_attr.keywords();
    let ty = &indexed_field.ty;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident (&self) -> #ty where #ty: core::clone::Clone {
            self.#field_ident.clone()
        }
    })
}
