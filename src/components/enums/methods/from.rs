use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::fields::{destructure_data, destructure_data_with_types, get_delimiter};

build_enum_name! { ConfigName, "from_{}" }
build_enum_doc! {
    ConfigDoc,
    "Creates a [`{}::{}`] variant from the associated data.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_from(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let input = destructure_data_with_types(fields, quote! { () }, Delimiter::Parenthesis, true);
    let destruct = destructure_data(fields, quote! {}, quote! {}, delimiter, true);

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let name = config.name;

    Ok(quote! {
        #[doc = #doc]
        #keywords fn #name #input -> Self {
            Self::#variant_ident #destruct
        }
    })
}
