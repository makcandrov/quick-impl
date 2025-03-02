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
    "Converts the instance into the associated data if the variant is [`{}::{}`]; otherwise, returns `Err(self)`.",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn enum_trait_try_from(
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
    let trait_ident = syn::Ident::new("TryFrom", attribute.ident.span());
    let method_ident = Ident::new("try_from", attribute.ident.span());

    let (impl_generics, ty_generics, where_clause) = context.generics.split_for_impl();
    let ident = context.ident;

    let content = quote! {
        impl #impl_generics ::core::convert:: #trait_ident <#ident #ty_generics> for #ty #where_clause {
            type Error = #ident #ty_generics;

            #[doc = #doc]
            #[inline]
            fn #method_ident (value: #ident #ty_generics) -> Result<Self, #ident #ty_generics> {
                match value {
                    #ident :: #variant_ident #destruct => Ok(#ret),
                    other => Err(other),
                }
            }
        }


    };

    Ok(content)
}
