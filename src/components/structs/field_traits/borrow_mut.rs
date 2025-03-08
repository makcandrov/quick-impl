use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::Attribute, config::Config, expand::Context, idents::config::CONFIG_DOC,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Mutably borrows from an owned value.";

pub fn expand_borrow_mut(
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

    let span = attribute.ident.span();
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("BorrowMut", span);
    let method_ident = Ident::new("borrow_mut", span);
    let field_ty = &indexed_field.ty;

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (&mut self) -> &mut #field_ty {
            &mut self.#field_ident
        }
    };

    Ok(context.in_impl(
        quote! { ::core::borrow::#trait_ident<#field_ty> for },
        &content,
        None,
    ))
}
