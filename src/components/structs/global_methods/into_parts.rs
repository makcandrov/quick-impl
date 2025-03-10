use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, LitStr};

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{destructure_types, to_indexed_field_iter, AloneDecoration},
};

const DEFAULT_NAME: &str = "into_parts";
const DEFAULT_DOC: &str = "Destructures the instance of [`{}`] into its fields values.";

pub fn expand_into_parts<'a>(
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

    let ret = destructure_types(
        fields,
        TokenStream::new(),
        quote! { () },
        AloneDecoration::None,
    );

    let mut destruct = TokenStream::new();

    for indexed_field in to_indexed_field_iter(fields) {
        let comma = (indexed_field.index != 0).then_some(quote! { , });
        let field = indexed_field.as_token();
        destruct.extend(quote! { #comma self.#field });
    }

    if fields.len() > 1 {
        destruct = quote! { ( #destruct )};
    }

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident (self) -> #ret {
            #destruct
        }
    })
}
