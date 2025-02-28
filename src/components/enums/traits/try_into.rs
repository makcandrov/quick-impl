use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::{
    destructure_data, destructure_types, get_delimiter, with_delimiter, RenameField,
};

build_enum_doc! {
    ConfigDoc,
    "Converts to the associated data if the variant is [`{}::{}`]. Otherwise, returns `Err(self)`.",
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

    let ty = destructure_types(fields, TokenStream::new(), quote! { () }, false);
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        true,
        RenameField::Auto,
    );
    let ret = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        false,
        RenameField::Auto,
    );

    let variant_ident = &variant.ident;
    let doc = &config.doc;
    let trait_ident = syn::Ident::new("TryInto", attribute.ident.span());
    let method_ident = Ident::new("try_into", attribute.ident.span());

    let content = quote! {
        type Error = Self;

        #[doc = #doc]
        #[inline]
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
