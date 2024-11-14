use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Variant};

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;

build_enum_name! { ConfigName, "is_{}" }
build_enum_doc! {
    ConfigDoc,
    "Returns `true` if the variant is [`{}::{}`]. Otherwise, returns `false`.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn enum_method_is(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let destruct = match &variant.fields {
        Fields::Named(_) => quote! { { .. } },
        Fields::Unnamed(_) => quote! { ( .. ) },
        Fields::Unit => TokenStream::new(),
    };

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();
    let doc = config.doc;
    let method_ident = config.name;

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(&self) -> bool {
            match self {
                Self::#variant_ident #destruct => true,
                _ => false,
            }
        }
    })
}
