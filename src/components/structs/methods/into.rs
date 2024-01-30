use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::fields::IndexedField;

build_enum_name! { ConfigName, "into_{}" }
build_enum_doc! {
    ConfigDoc,
    "Converts into the field `{1}` of [`{0}`].",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn struct_method_into(
    context: &Context,
    indexed_field: &IndexedField,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;

    let ty = &indexed_field.ty;
    let keywords = method_attr.keywords();
    let name = &config.name;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #keywords fn #name (self) -> #ty {
            self.#field_ident
        }
    })
}
