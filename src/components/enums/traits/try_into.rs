use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::{destructure_data, destructure_types, get_delimiter};

build_enum_doc! {
    ConfigDoc,
    "Converts into the associated data if it is the [`{}::{}`] variant. Otherwise, returns `Err(self)`.",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn enum_trait_try_into(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
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
    let doc = &config.doc;
    let trait_ident = syn::Ident::new("TryInto", attribute.ident.span());
    let method_ident = Ident::new("try_into", attribute.ident.span());

    let content = quote! {
        type Error = Self;

        #[doc = #doc]
        fn #method_ident (self) -> Result<#ty, Self> {
            match self {
                Self:: #variant_ident #destruct => Ok(#ret),
                other => Err(other),
            }
        }
    };

    Ok(context.in_impl(
        quote! { ::core::convert::#trait_ident<#ty> for },
        &content,
        None,
    ))
}
