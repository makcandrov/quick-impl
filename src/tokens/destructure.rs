use proc_macro2::{Delimiter, TokenStream};
use quote::{ToTokens, quote};
use syn::Field;

use super::with_delimiter;
use crate::tokens::indexed_field::field_rename;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum RenameField {
    #[default]
    Auto,
    Always,
    AlwaysIgnoreOriginal,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AloneDecoration {
    #[default]
    /// item
    None,
    /// (item)
    DelimitedNoComma,
    /// (item,)
    DelimitedWithComma,
}

impl AloneDecoration {
    pub fn apply(&self, tokens: TokenStream, delimiter: Delimiter) -> TokenStream {
        match self {
            AloneDecoration::None => tokens,
            AloneDecoration::DelimitedNoComma => with_delimiter(tokens, delimiter),
            AloneDecoration::DelimitedWithComma => with_delimiter(quote! { #tokens, }, delimiter),
        }
    }
}

pub fn destructure_types<'a, I>(
    fields: I,
    prefix: impl ToTokens,
    empty: impl ToTokens,
    alone: AloneDecoration,
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
        return alone.apply(res, Delimiter::Parenthesis);
    }

    for field in fields {
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
    alone: AloneDecoration,
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
        return alone.apply(res, delimiter);
    }

    let mut i = 1;
    for field in fields {
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
    alone: AloneDecoration,
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
        return alone.apply(res, delimiter);
    }

    let mut i = 1;
    for field in fields {
        let field_ident =
            if let Some(ident) = &field.ident { ident.clone() } else { field_rename(field, i) };
        let field_type = &field.ty;

        res.extend(quote! { , #field_ident: #field_type});
        i += 1
    }

    with_delimiter(res, delimiter)
}
