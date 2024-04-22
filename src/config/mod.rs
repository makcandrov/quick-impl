mod doc;
pub(crate) use doc::build_enum_doc;

mod name;
pub(crate) use name::build_enum_name;

pub trait Configurable {
    type Value;
    const IDENT: &'static str;

    fn default<'a>(
        context: &crate::expand::Context<'_>,
        attribute: &crate::attributes::Attribute,
        item: &crate::tokens::VariantOrField<'a>,
    ) -> syn::Result<Self::Value>;

    fn custom<'a>(item: &crate::tokens::VariantOrField<'a>, lit: &syn::Lit) -> syn::Result<Self::Value>;
}

macro_rules! build_config {
    ($config: ident, $(($name: ident, $t: ty, $default: expr),)*) => {
        #[derive(Clone)]
        struct $config {
            $($name: <$t as $crate::config::Configurable>::Value,)*
        }

        impl $config {
            pub fn new<'a>(
                context: &$crate::expand::Context<'_>,
                attribute: &$crate::attributes::Attribute,
                item: impl Into<$crate::tokens::VariantOrField<'a>>
            ) -> syn::Result<Self> {
                let item = Into::<$crate::tokens::VariantOrField<'a>>::into(item);

                #[derive(::core::default::Default)]
                struct ConfigInner {
                    $($name: ::std::option::Option<<$t as $crate::config::Configurable>::Value>,)*
                }

                let mut config = ConfigInner::default();

                match &attribute.config {
                    $crate::attributes::AttributeConfig::None => {},
                    $crate::attributes::AttributeConfig::Single(lit) => {
                        let _ = lit;
                        if false { }
                        $(
                            else if $default {
                                config.$name.replace(<$t as $crate::config::Configurable>::custom(&item, lit)?);
                            }
                        )*
                        else {
                            return ::syn::Result::Err(syn::Error::new_spanned(lit, "No default config available."));
                        }
                    },
                    $crate::attributes::AttributeConfig::Multiple(params) => {
                        for param in params {
                            match param.ident.to_string().as_str() {
                                $(<$t as $crate::config::Configurable>::IDENT => {
                                    if config.$name.replace(<$t as $crate::config::Configurable>::custom(&item, &param.literal)?).is_some() {
                                        return ::syn::Result::Err(::syn::Error::new_spanned(&param.ident, "Duplicate parameter."));
                                    }
                                },)*
                                _ => return ::syn::Result::Err(::syn::Error::new_spanned(&param.ident, "Unknown parameter.")),
                            }
                        }
                    },
                }

                Ok(Self {
                    $($name: {
                        if let ::std::option::Option::Some(value) = config.$name {
                            value
                        } else {
                            <$t as $crate::config::Configurable>::default(context, attribute, &item)?
                        }
                    },)*
                })
            }
        }
    }
}

pub(crate) use build_config;
