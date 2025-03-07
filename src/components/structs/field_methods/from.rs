use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::{Attribute, MethodAttribute};
use crate::config::{build_config, build_enum_doc, build_enum_name};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_name! { ConfigName, "from_{}" }
build_enum_doc! {
    ConfigDoc,
    "Creates an instance of [`{0}`] from the `{1}` field. Sets the other fields to their default value.",
}

build_config! {
    Config,
    (name, ConfigName, true),
    (doc, ConfigDoc, false),
}

pub fn expand_from<'a>(
    context: &'a Context,
    indexed_field: &'a IndexedField,
    attribute: &'a Attribute,
    method_attr: &'a MethodAttribute,
    indexed_fields: &'a Vec<IndexedField<'a>>,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let keywords = method_attr.keywords();
    let field_ty = &indexed_field.ty;
    let method_ident = &config.name;
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
        #[inline]
        #keywords fn #method_ident (#field_ident: #field_ty) -> Self #where_clause {
            #structure_creation
        }
    })
}
