use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Cheap mutable-to-mutable reference conversion.{0:.0}{1:.0}",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_as_mut(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("AsMut", attribute.ident.span());
    let method_ident = Ident::new("as_mut", attribute.ident.span());
    let field_ty = &indexed_field.ty;

    let content = quote! {
        #[doc = #doc]
        fn #method_ident (&mut self) -> &mut #field_ty {
            &mut self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { ::std::convert::#trait_ident<#field_ty> for }, &content))
}
