use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::IndexedField,
};

const DEFAULT_NAME: &str = "from_{}";
const DEFAULT_DOC: &str = "Creates an instance of [`{0}`] from the `{1}` field. Sets the other fields to their default value.";

pub fn expand_from<'a>(
    context: &'a Context,
    indexed_field: &'a IndexedField,
    attribute: &'a Attribute,
    method_attr: &'a MethodAttribute,
    indexed_fields: &'a Vec<IndexedField<'a>>,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, attribute.ident.span()),
        [&indexed_field.as_token().to_string()],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [
            &context.ident.to_string(),
            &indexed_field.as_token().to_string(),
        ],
    )?;

    config.finish()?;

    let keywords = method_attr.keywords();
    let field_ty = &indexed_field.ty;
    let field_ident = indexed_field.as_ident();

    let mut where_clause = if indexed_fields.len() > 1 {
        quote! { where }
    } else {
        TokenStream::new()
    };

    let mut other_fields = TokenStream::new();
    for other_indexed_field in indexed_fields {
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

    Ok(quote! {
        #[doc = #doc]
        #[must_use]
        #[inline]
        #keywords fn #method_ident (#field_ident: #field_ty) -> Self #where_clause {
            #structure_creation
        }
    })
}
