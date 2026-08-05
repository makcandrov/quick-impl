use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct, LitStr, parse2};

use crate::{
    config::Config,
    ctx::Context,
    idents::config::CONFIG_DOC,
    order::OrderTrait,
    tokens::{IndexedField, construct_defaulting_others},
};

const DEFAULT_DOC: &str = "Creates an instance of [`{0}`] from the `{1}` field, setting the remaining fields to their default values.";

pub fn expand_from(
    input: &ItemStruct,
    indexed_field: &IndexedField<'_>,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.name()],
    )?;

    config.finish()?;

    let field_ty = &indexed_field.ty;
    let field_ident = indexed_field.as_ident();
    let trait_ident = Ident::new("From", order.ident.span());
    let method_ident = Ident::new("from", order.ident.span());

    let (bounds, structure_creation) = construct_defaulting_others(&input.fields, indexed_field);
    let where_clause = parse2(quote! { where #bounds })?;

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (#field_ident: #field_ty) -> Self {
            #structure_creation
        }
    };

    Ok(input.in_impl(
        quote! { ::core::convert:: #trait_ident<#field_ty> for },
        &content,
        Some(where_clause),
    ))
}
