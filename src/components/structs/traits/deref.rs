use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Dereferences the value.{0:.0}{1:.0}",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_deref(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let deref_trait = Ident::new("Deref", attribute.ident.span());
    let name = Ident::new("deref", attribute.ident.span());

    let content = quote! {
        type Target = #field_type;

        #[doc = #doc]
        fn #name (&self) -> &Self::Target {
            &self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { ::core::ops::#deref_trait for }, &content))
}
