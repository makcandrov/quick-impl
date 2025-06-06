use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{
    config::Config, ctx::Context, idents::config::CONFIG_DOC, order::OrderTrait,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Cheap reference-to-reference conversion.";

pub fn expand_as_ref(
    input: &ItemStruct,
    indexed_field: &IndexedField<'_>,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.as_token().to_string()],
    )?;

    config.finish()?;

    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("AsRef", order.ident.span());
    let method_ident = Ident::new("as_ref", order.ident.span());
    let field_ty = &indexed_field.ty;

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (&self) -> &#field_ty {
            &self.#field_ident
        }
    };

    Ok(input.in_impl(quote! { ::core::convert::#trait_ident<#field_ty> for }, &content, None))
}
