use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident};

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Creates an instance of [`{0}`] from the `{1}` field. Sets the other fields to their default value.",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_from<'a>(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
    indexed_fields: &'a Vec<IndexedField<'a>>,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let field_ty = &indexed_field.ty;
    let field_ident = indexed_field.as_ident();
    let trait_ident = Ident::new("From", attribute.ident.span());
    let method_ident = Ident::new("from", attribute.ident.span());

    let mut where_clause = quote! { where };

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

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (#field_ident: #field_ty) -> Self {
            #structure_creation
        }
    };

    Ok(context.in_impl(
        quote! { ::core::convert:: #trait_ident<#field_ty> for },
        &content,
        Some(parse2(where_clause).unwrap()),
    ))
}
