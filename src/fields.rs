use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Field, Fields};

pub fn get_delimiter(fields: &Fields) -> Delimiter {
    match fields {
        Fields::Named(_) => Delimiter::Brace,
        Fields::Unnamed(_) => Delimiter::Parenthesis,
        Fields::Unit => Delimiter::Parenthesis,
    }
}

pub fn destructure_types<'a, I>(
    fields: I,
    prefix: TokenStream,
    empty: TokenStream,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty;
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
    prefix: TokenStream,
    delimiter: Delimiter,
    empty: TokenStream,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty;
    };

    let mut res = if let Some(ident) = &first.ident {
        quote! { #prefix #ident }
    } else {
        quote! { #prefix arg0 }
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
            Ident::new(&format!("arg{i}"), field.span())
        };

        res.extend(quote! { , #prefix #field_ident });
        i += 1
    }

    with_delimiter(res, delimiter)
}

pub fn destructure_data_with_types<'a, I>(
    fields: I,
    delimiter: Delimiter,
    empty: TokenStream,
    parenthesize_empty: bool,
) -> TokenStream
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut fields = fields.into_iter().peekable();

    let Some(first) = fields.next() else {
        return empty;
    };

    let first_type = &first.ty;

    let mut res = if let Some(ident) = &first.ident {
        quote! { #ident: #first_type }
    } else {
        quote! { arg0 }
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
            Ident::new(&format!("arg{i}"), field.span())
        };
        let field_type = &field.ty;

        res.extend(quote! { , #field_ident: #field_type});
        i += 1
    }

    with_delimiter(res, delimiter)
}

fn with_delimiter(input: TokenStream, delimiter: Delimiter) -> TokenStream {
    match delimiter {
        Delimiter::Parenthesis => quote! { ( #input ) },
        Delimiter::Brace => quote! { { #input } },
        Delimiter::Bracket => quote! { [ #input ] },
        Delimiter::None => quote! { #input },
    }
}
