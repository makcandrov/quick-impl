use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct, LitStr};

use crate::{
    attr::{Attr, AttrMethod},
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{AloneDecoration, destructure_types, to_indexed_field_iter},
};

const DEFAULT_NAME: &str = "into_parts";
const DEFAULT_DOC: &str = "Destructures the instance of [`{}`] into its fields values.";

pub fn expand_into_parts(
    input: &ItemStruct,
    attribute: &Attr,
    method_attr: &AttrMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config
        .get_lit_str_ident(CONFIG_NAME)?
        .unwrap_or_else(|| Ident::new(DEFAULT_NAME, attribute.ident.span()));

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string()],
    )?;

    config.finish()?;

    let keywords = method_attr.keywords();

    let ret = if input.fields.is_empty() {
        TokenStream::new()
    } else {
        let ret = destructure_types(
            &input.fields,
            TokenStream::new(),
            TokenStream::new(),
            AloneDecoration::None,
        );
        quote! { -> #ret }
    };

    let mut destruct = TokenStream::new();

    for indexed_field in to_indexed_field_iter(&input.fields) {
        let comma = (indexed_field.index != 0).then_some(quote! { , });
        let field = indexed_field.as_token();
        destruct.extend(quote! { #comma self.#field });
    }

    if input.fields.len() > 1 {
        destruct = quote! { ( #destruct )};
    }

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident (self) #ret {
            #destruct
        }
    })
}
