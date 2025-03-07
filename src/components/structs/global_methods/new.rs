use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Fields, Ident};

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::Config;
use crate::expand::Context;
use crate::tokens::{
    destructure_data, destructure_data_with_types, get_delimiter, with_delimiter, RenameField,
};

const DEFAULT_NAME: &str = "new";
const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] with the specified field values.";

pub fn expand_new<'a>(
    context: &'a Context,
    attribute: &'a Attribute,
    method_attr: &'a MethodAttribute,
    fields: &'a Fields,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some("name"))?;

    let method_ident = if let Some(lit_str) = config.get_lit_str("name")? {
        Ident::new(&lit_str.value(), lit_str.span())
    } else {
        Ident::new(DEFAULT_NAME, attribute.ident.span())
    };

    let doc = if let Some(lit_str) = config.get_lit_str("doc")? {
        let span = lit_str.span();
        let doc = lit_str.value();
        quote_spanned! {span=> #doc}
    } else {
        let doc = DEFAULT_DOC.replace("{}", &context.ident.to_string());
        quote! { #doc }
    };

    config.finish()?;

    let keywords = method_attr.keywords();
    let delimiter = get_delimiter(fields);

    let input = destructure_data_with_types(fields, quote! { () }, Delimiter::Parenthesis, true);
    let structure_creation = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        true,
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
