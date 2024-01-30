use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::fields::{destructure_data, destructure_types, get_delimiter};

build_enum_doc! {
    ConfigDoc,
    "Creates a [`{}::{}`] variant from the associated data.",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn enum_trait_from(context: &Context, variant: &Variant, attribute: &Attribute) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, quote! {}, quote! { () }, false);
    let destruct = destructure_data(fields, quote! {}, quote! {}, delimiter, true);
    let ret = destructure_data(fields, quote! {}, quote! { () }, Delimiter::Parenthesis, false);

    let variant_ident = &variant.ident;
    let doc = &config.doc;
    let from_trait = syn::Ident::new("From", attribute.ident.span());
    let name = Ident::new("from", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        fn #name (#ret: #ty) -> Self {
            Self::#variant_ident #destruct
        }
    };

    Ok(context.in_impl(quote! { ::core::convert::#from_trait<#ty> for }, &content))
}
