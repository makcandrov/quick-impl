use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::{
    destructure_data, destructure_types, get_delimiter, with_delimiter, RenameField,
};

build_enum_name! { ConfigName, "as_{}" }
build_enum_doc! {
    ConfigDoc,
    "Returns an immutable reference to the associated data if the variant is [`{}::{}`]; otherwise, returns `None`.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_as_ref(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, quote! { & }, quote! { () }, false);
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
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let method_ident = config.name;

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(&self) -> Option<#ty> {
            match self {
                Self::#variant_ident #destruct => Some(#ret),
                _ => None,
            }
        }
    })
}
