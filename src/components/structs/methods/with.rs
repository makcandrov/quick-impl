use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::fields::IndexedField;

build_enum_name! { ConfigName, "with_{}" }
build_enum_doc! {
    ConfigDoc,
    "Changes the field `{1}` of the given [`{0}`] variable.",
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

    let ty = &indexed_field.ty;
    let argument_ident = indexed_field.as_ident();
    let keywords = method_attr.keywords();
    let name = &config.name;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #keywords fn #name (mut self, #argument_ident: #ty) -> Self {
            self.#field_ident = #argument_ident;
            self
        }
    })
}
