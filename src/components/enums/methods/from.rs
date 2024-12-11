use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::{destructure_data, destructure_data_with_types, get_delimiter, with_delimiter};

build_enum_name! { ConfigName, "from_{}" }
build_enum_doc! {
    ConfigDoc,
    "Creates a [`{}::{}`] variant from the provided data.",
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
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        true,
    );

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let method_ident = config.name;

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident #input -> Self {
            Self::#variant_ident #destruct
        }
    })
}
