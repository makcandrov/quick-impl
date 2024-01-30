use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{Field, Ident};

use super::with_delimiter;
use crate::idents::ARGUMENT;

pub fn destructure_types<'a, I>(
    fields: I,
    prefix: impl ToTokens,
    empty: impl ToTokens,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty.to_token_stream();
    };

    let first_type = &first.ty;

    let mut res = quote! { #prefix #first_type };

    if fields.peek().is_none() {
        return if parenthesize_empty {
            quote! { ( #res ) }
        } else {
            res
        };
    }

    while let Some(field) = fields.next() {
        let field_type = &field.ty;
        res.extend(quote! { , #prefix #field_type });
    }

    quote! { ( #res ) }
}

pub fn destructure_data<'a, I>(
    fields: I,
    prefix: impl ToTokens,
    empty: impl ToTokens,
    delimiter: Delimiter,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty.to_token_stream();
    };

    let mut res = if let Some(ident) = &first.ident {
        quote! { #prefix #ident }
    } else {
        let first_ident = Ident::new(&format!("{ARGUMENT}0",), first.span());
        quote! { #prefix #first_ident }
    };

    if fields.peek().is_none() {
        return if parenthesize_empty {
            with_delimiter(res, delimiter)
        } else {
            res
        };
    }

    let mut i = 1;
    while let Some(field) = fields.next() {
        let field_ident = if let Some(ident) = &field.ident {
            ident.clone()
        } else {
            Ident::new(&format!("{ARGUMENT}{i}"), field.span())
        };

        res.extend(quote! { , #prefix #field_ident });
        i += 1
    }

    with_delimiter(res, delimiter)
}

pub fn destructure_data_with_types<'a, I>(
    fields: I,
    empty: impl ToTokens,
    delimiter: Delimiter,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty.to_token_stream();
    };

    let first_type = &first.ty;

    let mut res = if let Some(ident) = &first.ident {
        quote! { #ident: #first_type }
    } else {
        let first_ident = Ident::new(&format!("{ARGUMENT}0",), first.span());
        quote! { #first_ident: #first_type }
    };

    if fields.peek().is_none() {
        return if parenthesize_empty {
            with_delimiter(res, delimiter)
        } else {
            res
        };
    }

    let mut i = 1;
    while let Some(field) = fields.next() {
        let field_ident = if let Some(ident) = &field.ident {
            ident.clone()
        } else {
            Ident::new(&format!("{ARGUMENT}{i}"), field.span())
        };
        let field_type = &field.ty;

        res.extend(quote! { , #field_ident: #field_type});
        i += 1
    }

    with_delimiter(res, delimiter)
}
