use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{
    config::Config,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    order::OrderMethod,
    tokens::{IndexedField, construct_defaulting_others},
};

const DEFAULT_NAME: &str = "from_{}";
const DEFAULT_DOC: &str = "Creates an instance of [`{0}`] from the `{1}` field, setting the remaining fields to their default values.";

pub fn expand_from(
    input: &ItemStruct,
    indexed_field: &IndexedField,
    order: &OrderMethod,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, order.ident.span()),
        [&indexed_field.name()],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.name()],
    )?;

    config.finish()?;

    let keywords = order.keywords();
    let field_ty = &indexed_field.ty;
    let field_ident = indexed_field.as_ident();

    let (bounds, structure_creation) = construct_defaulting_others(&input.fields, indexed_field);
    let where_clause = (!bounds.is_empty()).then(|| quote! { where #bounds });

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident (#field_ident: #field_ty) -> Self #where_clause {
            #structure_creation
        }
    })
}
