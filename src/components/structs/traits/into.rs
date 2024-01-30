use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::fields::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Converts into the field `{1}` of [`{0}`].",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_into(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let deref_trait = Ident::new("Into", attribute.ident.span());
    let name = Ident::new("into", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        fn #name (self) -> #field_type {
            self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { ::core::convert::#deref_trait<#field_type> for }, &content))
}
