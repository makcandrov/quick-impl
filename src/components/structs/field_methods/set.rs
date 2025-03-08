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

const DEFAULT_NAME: &str = "set_{}";
const DEFAULT_DOC: &str = "A setter for the `{1}` field of [`{0}`]. Returns the previous value.";

pub fn expand_set(
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
    let arg_ident = indexed_field.as_ident();
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident (&mut self, #arg_ident: #ty) -> #ty {
            ::core::mem::replace(&mut self.#field_ident, #arg_ident)
        }
    })
}
