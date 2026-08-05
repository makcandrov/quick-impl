use proc_macro2::{Delimiter, TokenStream};
use quote::{ToTokens, quote};
use syn::{Field, Fields};

use super::{get_delimiter, with_delimiter};
use crate::tokens::indexed_field::{IndexedField, field_rename, to_indexed_field_iter};

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

    for (i, field) in (1..).zip(fields) {
        let ext = if let Some(ident) = &field.ident {
            match rename {
                RenameField::Auto => quote! { , #prefix #ident },
                RenameField::Always => {
                    let rename_ident = field_rename(field, i);
                    quote! { , #ident: #prefix #rename_ident }
                }
                RenameField::AlwaysIgnoreOriginal => {
                    let rename_ident = field_rename(field, i);
                    quote! { , #prefix #rename_ident}
                }
            }
        } else {
            let ident = field_rename(field, i);
            quote! { , #prefix #ident }
        };

        res.extend(ext);
    }

    with_delimiter(res, delimiter)
}

/// Builds the expression constructing `Self` from `kept`, defaulting every other field, along with
/// the `Default` bounds those fields require.
///
/// Fields are emitted in declaration order, which tuple structs rely on.
pub fn construct_defaulting_others(
    fields: &Fields,
    kept: &IndexedField<'_>,
) -> (TokenStream, TokenStream) {
    let kept_ident = kept.as_ident();
    let mut bounds = TokenStream::new();
    let mut values = TokenStream::new();

    for field in to_indexed_field_iter(fields) {
        let value = if field.index == kept.index {
            quote! { #kept_ident }
        } else {
            let ty = &field.ty;
            bounds.extend(quote! { #ty: ::core::default::Default, });
            quote! { ::core::default::Default::default() }
        };

        if let Some(ident) = &field.ident {
            values.extend(quote! { #ident: #value, });
        } else {
            values.extend(quote! { #value, });
        }
    }

    let construction = with_delimiter(values, get_delimiter(fields));
    (bounds, quote! { Self #construction })
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

    for (i, field) in (1..).zip(fields) {
        let field_ident = if let Some(ident) = &field.ident {
            ident.clone()
        } else {
            field_rename(field, i)
        };
        let field_type = &field.ty;

        res.extend(quote! { , #field_ident: #field_type});
    }

    with_delimiter(res, delimiter)
}
