macro_rules! build_enum_name {
    ($t: ident, $pat: literal $(,)?) => {
        struct $t;

        impl $t {
            fn get_item_name<'a>(item: &$crate::tokens::VariantOrField<'a>) -> String {
                match item {
                    $crate::tokens::VariantOrField::Variant(variant) => {
                        use convert_case::Casing;
                        variant.ident.to_string().to_case(::convert_case::Case::Snake)
                    },
                    $crate::tokens::VariantOrField::Field(field) => field.as_token().to_string(),
                }
            }
        }

        impl $crate::config::Configurable for $t {
            type Value = ::syn::Ident;

            const IDENT: &'static str = $crate::idents::config::CONFIG_NAME;

            fn default<'a>(
                _: &$crate::expand::Context<'_>,
                attribute: &$crate::attributes::Attribute,
                item: &$crate::tokens::VariantOrField<'a>,
            ) -> syn::Result<Self::Value> {
                let item_name = match item {
                    $crate::tokens::VariantOrField::Variant(variant) => {
                        use convert_case::Casing;
                        variant.ident.to_string().to_case(convert_case::Case::Snake)
                    },
                    $crate::tokens::VariantOrField::Field(field) => field.as_token().to_string(),
                };

                ::syn::Result::Ok(::syn::Ident::new(
                    &format!($pat, item_name),
                    attribute.ident.span(),
                ))
            }
            fn custom<'a>(item: &$crate::tokens::VariantOrField<'a>, lit: &::syn::Lit) -> ::syn::Result<Self::Value> {
                let ::syn::Lit::Str(lit_str) = lit else {
                    return ::syn::Result::Err(::syn::Error::new_spanned(lit, "Expected string literal."));
                };
                let name = lit_str.value().replace("{}", &Self::get_item_name(item));
                ::syn::Result::Ok(::syn::Ident::new(&name, lit_str.span()))
            }
        }
    };
}

pub(crate) use build_enum_name;
