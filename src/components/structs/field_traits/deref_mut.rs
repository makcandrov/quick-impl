use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{
    config::Config, ctx::Context, idents::config::CONFIG_DOC, order::OrderTrait,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Mutably dereferences the value.";

pub fn expand_deref_mut(
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
    let trait_ident = Ident::new("DerefMut", order.ident.span());
    let method_ident = Ident::new("deref_mut", order.ident.span());

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (&mut self) -> &mut <Self as ::core::ops::Deref>::Target {
            &mut self.#field_ident
        }
    };

    Ok(input.in_impl(quote! { ::core::ops::#trait_ident for }, &content, None))
}
