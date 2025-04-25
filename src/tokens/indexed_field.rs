use core::ops::Deref;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::{Field, Ident, Index};

use crate::idents::ARGUMENT;

#[derive(Clone)]
pub struct IndexedField<'a> {
    pub field: &'a Field,
    pub index: usize,
}

impl<'a> Deref for IndexedField<'a> {
    type Target = &'a Field;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<'a> IndexedField<'a> {
    pub const fn new(field: &'a Field, index: usize) -> Self {
        Self { field, index }
    }

    pub fn as_token(&self) -> TokenStream {
        self.field
            .ident
            .as_ref()
            .map(|x| x.to_token_stream())
            .unwrap_or_else(|| Index::from(self.index).to_token_stream())
    }

    pub fn as_ident(&self) -> Ident {
        self.field
            .ident
            .clone()
            .unwrap_or_else(|| format_ident!("{}{}", ARGUMENT, self.index))
    }
}

pub fn to_indexed_field_iter<'a, I>(fields: I) -> impl Iterator<Item = IndexedField<'a>>
where
    I: IntoIterator<Item = &'a Field>,
{
    fields
        .into_iter()
        .enumerate()
        .map(|(index, field)| IndexedField::new(field, index))
}
