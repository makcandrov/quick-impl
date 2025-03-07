use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Fields, Ident};

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::Config;
use crate::expand::Context;
use crate::tokens::{destructure_types, to_indexed_field_iter};

const DEFAULT_NAME: &str = "into_parts";
const DEFAULT_DOC: &str = "Destructures the instance of [`{}`] into its fields values.";

pub fn expand_into_parts<'a>(
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

    let ret = destructure_types(fields, TokenStream::new(), TokenStream::new(), false);

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
