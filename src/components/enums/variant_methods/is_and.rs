use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{ItemEnum, LitStr, Variant};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_types, get_delimiter,
        with_delimiter,
    },
    utils::to_snake_case,
};

const DEFAULT_NAME: &str = "is_{}_and";
const DEFAULT_DOC: &str = "Returns `true` if the variant is [`{}::{}`] and its associated data matches the predicate; otherwise, returns `false`.";

pub fn expand_is_and(
    input: &ItemEnum,
    variant: &Variant,
    order: &OrderMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, order.ident.span()),
        [&to_snake_case(&variant.ident.to_string())],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let need_rename =
        fields.iter().any(|field| field.ident.as_ref().is_some_and(|ident| ident == "f"));

    let ty =
        destructure_types(fields, quote! { & }, quote! { () }, AloneDecoration::DelimitedNoComma);

    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        if need_rename { RenameField::Always } else { RenameField::Auto },
    );
    let args = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedNoComma,
        if need_rename { RenameField::AlwaysIgnoreOriginal } else { RenameField::Auto },
    );

    let variant_ident = &variant.ident;
    let keywords = order.keywords();

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident(&self, f: impl ::core::ops::FnOnce #ty -> bool) -> bool {
            match self {
                Self::#variant_ident #destruct => f #args,
                _ => false,
            }
        }
    })
}
