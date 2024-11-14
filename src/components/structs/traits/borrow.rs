use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Immutably borrows from an owned value.{0:.0}{1:.0}",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_borrow(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let span = attribute.ident.span();
    let doc = &config.doc;
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("Borrow", span);
    let method_ident = Ident::new("borrow", span);
    let field_ty = &indexed_field.ty;

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (&self) -> &#field_ty {
            &self.#field_ident
        }
    };

    Ok(context.in_impl(
        quote! { ::core::borrow::#trait_ident<#field_ty> for },
        &content,
        None,
    ))
}
