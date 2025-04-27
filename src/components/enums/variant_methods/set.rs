use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{ItemEnum, LitStr, Variant};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_data_with_types, get_delimiter,
        with_delimiter,
    },
    utils::to_snake_case,
};

const DEFAULT_NAME: &str = "set_{}";
const DEFAULT_DOC: &str = "Replaces the current instance with a new instance of the [`{}::{}`] variant, returning the original instance.";

pub fn expand_set(
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

    let input = destructure_data_with_types(
        fields,
        TokenStream::new(),
        Delimiter::None,
        AloneDecoration::None,
    );
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        RenameField::Auto,
    );

    let variant_ident = &variant.ident;
    let keywords = order.keywords();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(&mut self, #input) -> Self {
            ::core::mem::replace(self, Self:: #variant_ident #destruct )
        }
    })
}
