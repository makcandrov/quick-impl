use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::{destructure_data, destructure_types, get_delimiter};

build_enum_name! { ConfigName, "into_{}" }
build_enum_doc! {
    ConfigDoc,
    "onverts to the associated data if the variant is [`{}::{}`]. Otherwise, returns `None`.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_into(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, quote! {}, quote! { () }, false);
    let destruct = destructure_data(fields, quote! {}, quote! {}, delimiter, true);
    let ret = destructure_data(
        fields,
        quote! {},
        quote! { () },
        Delimiter::Parenthesis,
        false,
    );

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let method_ident = config.name;

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(self) -> Option<#ty> {
            match self {
                Self::#variant_ident #destruct => Some(#ret),
                _ => None,
            }
        }
    })
}
