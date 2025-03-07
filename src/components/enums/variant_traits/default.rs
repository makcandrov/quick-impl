use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Variant;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::{get_delimiter, with_delimiter};

build_enum_doc! {
    ConfigDoc,
    "Creates a [`{}::{}`] variant with the default associated data.",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn expand_default(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, variant)?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let variant_ident = &variant.ident;
    let doc = &config.doc;
    let trait_ident = syn::Ident::new("Default", attribute.ident.span());
    let method_ident = Ident::new("default", attribute.ident.span());

    let default = quote! { ::core::default:: #trait_ident :: #method_ident () };
    let mut default_data = TokenStream::new();
    let mut where_clause = quote! { where };

    for field in &variant.fields {
        let field_ty = &field.ty;
        if let Some(ident) = &field.ident {
            default_data.extend(quote! { #ident : });
        }
        default_data.extend(quote! { #default, });
        where_clause.extend(quote! { #field_ty : ::core::default:: #trait_ident, });
    }
    let default_data = with_delimiter(default_data, delimiter);

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident () -> Self {
            Self::#variant_ident #default_data
        }
    };

    Ok(context.in_impl(
        quote! { ::core::default::#trait_ident for },
        &content,
        Some(syn::parse2(where_clause).unwrap()),
    ))
}
