use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::Attribute, config::Config, expand::Context, idents::config::CONFIG_DOC,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Converts into the `{1}` field of [`{0}`].";

pub fn expand_into(
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
    let trait_ident = Ident::new("Into", attribute.ident.span());
    let method_ident = Ident::new("into", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (self) -> #field_type {
            self.#field_ident
        }
    };

    Ok(context.in_impl(
        quote! { ::core::convert::#trait_ident<#field_type> for },
        &content,
        None,
    ))
}
