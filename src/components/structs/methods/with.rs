use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_name! { ConfigName, "with_{}" }
build_enum_doc! {
    ConfigDoc,
    "Returns an instance of [`{0}`] with the `{1}` field modified.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn struct_method_with(
    context: &Context,
    indexed_field: &IndexedField,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let keywords = method_attr.keywords();
    let ty = &indexed_field.ty;
    let arg_ident = indexed_field.as_ident();
    let method_ident = &config.name;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #[must_use]
        #keywords fn #method_ident (mut self, #arg_ident: #ty) -> Self {
            self.#field_ident = #arg_ident;
            self
        }
    })
}
