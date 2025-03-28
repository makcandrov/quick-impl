use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::Attribute, config::Config, expand::Context, idents::config::CONFIG_DOC,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Dereferences the value.";

pub fn expand_deref(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [
            &context.ident.to_string(),
            &indexed_field.as_token().to_string(),
        ],
    )?;

    config.finish()?;

    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("Deref", attribute.ident.span());
    let method_ident = Ident::new("deref", attribute.ident.span());

    let content = quote! {
        type Target = #field_type;

        #[doc = #doc]
        #[inline]
        fn #method_ident (&self) -> &Self::Target {
            &self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { ::core::ops::#trait_ident for }, &content, None))
}
