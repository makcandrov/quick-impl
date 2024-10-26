macro_rules! build_enum_doc {
    ($t: ident, $pat: literal $(,)?) => {
        struct $t;

        impl $crate::config::Configurable for $t {
            type Value = String;

            const IDENT: &'static str = $crate::idents::config::CONFIG_DOC;

            fn default<'a>(
                context: &$crate::expand::Context<'_>,
                _: &$crate::attributes::Attribute,
                item: &$crate::tokens::VariantOrField<'a>,
            ) -> syn::Result<Self::Value> {
                match item {
                    $crate::tokens::VariantOrField::Variant(variant) => {
                        ::syn::Result::Ok(format!($pat, &context.ident, variant.ident))
                    }
                    $crate::tokens::VariantOrField::Field(field) => ::syn::Result::Ok(format!(
                        $pat,
                        &context.ident,
                        field.as_token().to_string()
                    )),
                }
            }

            fn custom<'a>(
                _: &$crate::tokens::VariantOrField<'a>,
                lit: &::syn::Lit,
            ) -> syn::Result<Self::Value> {
                let ::syn::Lit::Str(lit_str) = lit else {
                    return ::syn::Result::Err(syn::Error::new_spanned(
                        lit,
                        "Expected string literal.",
                    ));
                };
                ::syn::Result::Ok(lit_str.value())
            }
        }
    };
}

pub(crate) use build_enum_doc;
