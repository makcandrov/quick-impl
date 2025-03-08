use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Field, Ident};

use super::with_delimiter;
use crate::idents::ARGUMENT;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum RenameField {
    #[default]
    Auto,
    Always,
    AlwaysIgnoreOriginal,
}

pub fn destructure_types<'a, I>(
    fields: I,
    prefix: impl ToTokens,
    empty: impl ToTokens,
    parenthesize_alone: bool,
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
        return if parenthesize_alone {
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
    parenthesize_alone: bool,
    rename: RenameField,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty.to_token_stream();
    };

    let mut res = if let Some(ident) = &first.ident {
        match rename {
            RenameField::Auto => quote! { #prefix #ident },
            RenameField::Always => {
                let rename_ident = field_rename(first, 0);
                quote! { #ident: #prefix #rename_ident }
            }
            RenameField::AlwaysIgnoreOriginal => {
                let rename_ident = field_rename(first, 0);
                quote! { #prefix #rename_ident }
            }
        }
    } else {
        let first_ident = field_rename(first, 0);
        quote! { #prefix #first_ident }
    };

    if fields.peek().is_none() {
        return if parenthesize_alone {
            with_delimiter(res, delimiter)
        } else {
            res
        };
    }

    let mut i = 1;
    while let Some(field) = fields.next() {
        let ext = if let Some(ident) = &field.ident {
            match rename {
                RenameField::Auto => quote! { , #prefix #ident },
                RenameField::Always => {
                    let rename_ident = field_rename(field, i);
                    quote! { , #ident: #prefix #rename_ident }
                }
                RenameField::AlwaysIgnoreOriginal => {
                    let rename_ident = field_rename(first, i);
                    quote! { , #prefix #rename_ident }
                }
            }
        } else {
            let ident = field_rename(field, i);
            quote! { , #prefix #ident }
        };

        res.extend(ext);
        i += 1
    }

    with_delimiter(res, delimiter)
}

pub fn destructure_data_with_types<'a, I>(
    fields: I,
    empty: impl ToTokens,
    delimiter: Delimiter,
    parenthesize_alone: bool,
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
        let first_ident = field_rename(first, 0);
        quote! { #first_ident: #first_type }
    };

    if fields.peek().is_none() {
        return if parenthesize_alone {
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
            field_rename(field, i)
        };
        let field_type = &field.ty;

        res.extend(quote! { , #field_ident: #field_type});
        i += 1
    }

    with_delimiter(res, delimiter)
}

fn field_rename(field: &Field, index: usize) -> Ident {
    Ident::new(&format!("{ARGUMENT}{index}"), field.span())
}
