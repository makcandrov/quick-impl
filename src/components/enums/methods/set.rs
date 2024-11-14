use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::{destructure_data, destructure_data_with_types, get_delimiter};

build_enum_name! { ConfigName, "set_{}" }
build_enum_doc! {
    ConfigDoc,
    "Replaces the current instance with a new instance of the [`{}::{}`] variant, returning the original instance.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_set(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let input = destructure_data_with_types(fields, TokenStream::new(), Delimiter::None, false);
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        TokenStream::new(),
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
        #keywords fn #method_ident(&mut self, #input) -> Self {
            ::core::mem::replace(self, Self:: #variant_ident #destruct )
        }
    })
}
