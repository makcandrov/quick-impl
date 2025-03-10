use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, LitStr};

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{
        destructure_data, destructure_data_with_types, get_delimiter, with_delimiter,
        AloneDecoration, RenameField,
    },
};

const DEFAULT_NAME: &str = "new";
const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] with the specified field values.";

pub fn expand_new<'a>(
    context: &'a Context,
    attribute: &'a Attribute,
    method_attr: &'a MethodAttribute,
    fields: &'a Fields,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config
        .get_lit_str_ident(CONFIG_NAME)?
        .unwrap_or_else(|| Ident::new(DEFAULT_NAME, attribute.ident.span()));

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&context.ident.to_string()],
    )?;

    config.finish()?;

    let keywords = method_attr.keywords();
    let delimiter = get_delimiter(fields);

    let input = destructure_data_with_types(
        fields,
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedNoComma,
    );
    let structure_creation = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        RenameField::Auto,
    );

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident #input -> Self {
            Self #structure_creation
        }
    })
}
