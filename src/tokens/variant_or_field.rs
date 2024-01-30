use syn::Variant;

use super::IndexedField;

pub enum VariantOrField<'a> {
    Variant(&'a Variant),
    Field(&'a IndexedField<'a>),
}

impl<'a> From<&'a Variant> for VariantOrField<'a> {
    fn from(variant: &'a Variant) -> Self {
        Self::Variant(variant)
    }
}

impl<'a> From<&'a IndexedField<'a>> for VariantOrField<'a> {
    fn from(field: &'a IndexedField<'a>) -> Self {
        Self::Field(field)
    }
}
