use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{ItemEnum, LitStr, Variant};

use crate::{
    config::Config,
    ctx::Context,
    idents::config::CONFIG_DOC,
    order::OrderTrait,
    tokens::{get_delimiter, with_delimiter},
};

const DEFAULT_DOC: &str = "Creates a [`{}::{}`] variant with the default associated data.";

pub fn expand_default(
    input: &ItemEnum,
    variant: &Variant,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let variant_ident = &variant.ident;
    let trait_ident = Ident::new("Default", order.ident.span());
    let method_ident = Ident::new("default", order.ident.span());

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

    Ok(input.in_impl(
        quote! { ::core::default::#trait_ident for },
        &content,
        Some(syn::parse2(where_clause).unwrap()),
    ))
}
