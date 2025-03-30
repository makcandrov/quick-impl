use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, LitStr};

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{
        destructure_data, destructure_types, get_delimiter, with_delimiter, AloneDecoration,
        RenameField,
    },
};

const DEFAULT_NAME: &str = "from_tuple";
const DEFAULT_DOC: &str =
    "Constructs a new instance of [`{}`] from a tuple with the specified field values .";

pub fn expand_from_tuple<'a>(
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

    let input = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedWithComma,
        RenameField::Auto,
    );
    let tuple_ty = destructure_types(
        fields,
        TokenStream::new(),
        quote! { () },
        AloneDecoration::DelimitedWithComma,
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
        #[must_use]
        #[inline]
        #keywords fn #method_ident (#input: #tuple_ty) -> Self {
            Self #structure_creation
        }
    })
}
