use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{LitStr, Variant};

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{
        destructure_data, destructure_data_with_types, get_delimiter, with_delimiter,
        AloneDecoration, RenameField,
    },
    utils::to_snake_case,
};

const DEFAULT_NAME: &str = "from_{}";
const DEFAULT_DOC: &str = "Creates a [`{}::{}`] variant from the provided data.";

pub fn expand_from(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, attribute.ident.span()),
        [&to_snake_case(&variant.ident.to_string())],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&context.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let input = destructure_data_with_types(
        fields,
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedNoComma,
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
    let keywords = method_attr.keywords();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident #input -> Self {
            Self::#variant_ident #destruct
        }
    })
}
