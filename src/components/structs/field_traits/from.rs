use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemStruct, LitStr, parse2};

use crate::{
    config::Config,
    ctx::Context,
    idents::config::CONFIG_DOC,
    order::OrderTrait,
    tokens::{IndexedField, to_indexed_field_iter},
};

const DEFAULT_DOC: &str = "Creates an instance of [`{0}`] from the `{1}` field. Sets the other fields to their default value.";

pub fn expand_from(
    input: &ItemStruct,
    indexed_field: &IndexedField<'_>,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.as_token().to_string()],
    )?;

    config.finish()?;

    let field_ty = &indexed_field.ty;
    let field_ident = indexed_field.as_ident();
    let trait_ident = Ident::new("From", order.ident.span());
    let method_ident = Ident::new("from", order.ident.span());

    let mut where_clause = quote! { where };

    let mut other_fields = TokenStream::new();
    for other_indexed_field in to_indexed_field_iter(&input.fields) {
        if other_indexed_field.index == indexed_field.index {
            continue;
        }
        let other_field_ident = &other_indexed_field.as_ident();
        let other_field_ty = &other_indexed_field.ty;
        where_clause.extend(quote! {
            #other_field_ty: ::core::default::Default,
        });

        if other_indexed_field.ident.is_some() {
            other_fields
                .extend(quote! { #other_field_ident: ::core::default::Default::default(), });
        } else {
            other_fields.extend(quote! { ::core::default::Default::default(), });
        }
    }

    let structure_creation = if let Some(field_ident) = &indexed_field.ident {
        quote! {
            Self { #field_ident, #other_fields }
        }
    } else {
        quote! {
            Self ( #field_ident, #other_fields )
        }
    };

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
        Some(parse2(where_clause).unwrap()),
    ))
}
