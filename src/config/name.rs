macro_rules! build_enum_name {
    ($t: ident, $pat: literal $(,)?) => {
        struct $t;

        impl $crate::config::Configurable for $t {
            type Value = ::syn::Ident;

            const IDENT: &'static str = crate::idents::config::CONFIG_NAME;

            fn default<'a>(
                _: &$crate::expand::Context<'_>,
                attribute: &$crate::attributes::Attribute,
                item: &$crate::tokens::VariantOrField<'a>,
            ) -> syn::Result<Self::Value> {
                match item {
                    $crate::tokens::VariantOrField::Variant(variant) => {
                        use convert_case::Casing;

                        let variant_name_snake_case = variant.ident.to_string().to_case(convert_case::Case::Snake);
                        ::syn::Result::Ok(::syn::Ident::new(
                            &format!($pat, variant_name_snake_case),
                            attribute.ident.span(),
                        ))
                    },
                    $crate::tokens::VariantOrField::Field(field) => ::syn::Result::Ok(::syn::Ident::new(
                        &format!($pat, field.as_token().to_string()),
                        attribute.ident.span(),
                    )),
                }
            }

            fn custom(lit: &::syn::Lit) -> ::syn::Result<Self::Value> {
                let ::syn::Lit::Str(lit_str) = lit else {
                    return ::syn::Result::Err(::syn::Error::new_spanned(lit, "Expected string literal."));
                };
                ::syn::Result::Ok(::syn::Ident::new(&lit_str.value(), lit_str.span()))
            }
        }
    };
}

pub(crate) use build_enum_name;
