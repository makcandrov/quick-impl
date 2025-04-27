use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct, LitStr};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_data_with_types, get_delimiter,
        with_delimiter,
    },
};

const DEFAULT_NAME: &str = "new";
const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] with the specified field values.";

pub fn expand_new(input: &ItemStruct, order: &OrderMethod) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, Some(CONFIG_NAME))?;

    let method_ident = config
        .get_lit_str_ident(CONFIG_NAME)?
        .unwrap_or_else(|| Ident::new(DEFAULT_NAME, order.ident.span()));

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string()],
    )?;

    config.finish()?;

    let keywords = order.keywords();
    let delimiter = get_delimiter(&input.fields);

    let input_tt = destructure_data_with_types(
        &input.fields,
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedNoComma,
    );
    let structure_creation = destructure_data(
        &input.fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        RenameField::Auto,
    );

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident #input_tt -> Self {
            Self #structure_creation
        }
    })
}
