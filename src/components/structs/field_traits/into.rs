use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Converts into the `{1}` field of [`{0}`].",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn expand_into(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
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
