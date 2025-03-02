use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::{
    destructure_data, destructure_types, get_delimiter, with_delimiter, RenameField,
};

build_enum_name! { ConfigName, "is_{}_and" }
build_enum_doc! {
    ConfigDoc,
    "Returns `true` if the variant is [`{}::{}`] and its associated data matches the predicate; otherwise, returns `false`.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_is_and(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let need_rename = fields.iter().any(|field| {
        field
            .ident
            .as_ref()
            .is_some_and(|ident| ident.to_string() == "f")
    });

    let ty = destructure_types(fields, quote! { & }, quote! { () }, true);

    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        true,
        if need_rename {
            RenameField::Always
        } else {
            RenameField::Auto
        },
    );
    let args = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        true,
        if need_rename {
            RenameField::AlwaysIgnoreOriginal
        } else {
            RenameField::Auto
        },
    );

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let method_ident = config.name;

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(&self, f: impl FnOnce #ty -> bool) -> bool {
            match self {
                Self::#variant_ident #destruct => f #args,
                _ => false,
            }
        }
    })
}
