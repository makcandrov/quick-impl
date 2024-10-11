use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_name! { ConfigName, "set_{}" }
build_enum_doc! {
    ConfigDoc,
    "A setter for the `{1}` field of [`{0}`]. Returns the previous value.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn struct_method_set(
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
        #keywords fn #method_ident (&mut self, #arg_ident: #ty) -> #ty {
            ::core::mem::replace(&mut self.#field_ident, #arg_ident)
        }
    })
}
