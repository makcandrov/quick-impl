use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct};

use crate::{
    config::Config,
    ctx::Context,
    idents::config::CONFIG_DOC,
    order::OrderTrait,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_types, get_delimiter,
        with_delimiter,
    },
};

const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] from a tuple of its fields values.";

pub fn expand_from(input: &ItemStruct, order: &OrderTrait) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_lit_str_tokens(CONFIG_DOC)?.unwrap_or_else(|| {
        let doc = DEFAULT_DOC.replace("{}", &input.ident.to_string());
        quote! { #doc }
    });

    config.finish()?;

    let delimiter = get_delimiter(&input.fields);

    let ty =
        destructure_types(&input.fields, TokenStream::new(), quote! { () }, AloneDecoration::None);
    let destruct = destructure_data(
        &input.fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        RenameField::Auto,
    );
    let arg = destructure_data(
        &input.fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::None,
        RenameField::Auto,
    );

    let trait_ident = Ident::new("From", order.ident.span());
    let method_ident = Ident::new("from", order.ident.span());

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (#arg: #ty) -> Self {
            Self #destruct
        }
    };

    let mut result =
        input.in_impl(quote! { ::core::convert::#trait_ident<#ty> for }, &content, None);

    // If there is exactly one field of type T, we need to implement both `From<T>` and
    // `From<(T,)>`.
    if input.fields.len() == 1 {
        let ty = quote! { (#ty,) };
        let arg = quote! { (#arg,) };

        let content = quote! {
            #[doc = #doc]
            #[inline]
            fn #method_ident (#arg: #ty) -> Self {
                Self #destruct
            }
        };

        result.extend(input.in_impl(
            quote! { ::core::convert::#trait_ident<#ty> for },
            &content,
            None,
        ));
    }

    Ok(result)
}
