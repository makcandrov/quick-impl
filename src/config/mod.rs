mod doc;
use std::collections::BTreeMap;

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

    fn custom<'a>(
        item: &crate::tokens::VariantOrField<'a>,
        lit: &syn::Lit,
    ) -> syn::Result<Self::Value>;
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
                    $($name: ::core::option::Option<<$t as $crate::config::Configurable>::Value>,)*
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
                        if let ::core::option::Option::Some(value) = config.$name {
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

use proc_macro2::Span;
use syn::{Lit, LitStr};

use crate::attributes::AttributeConfig;

#[must_use]
pub struct Config(BTreeMap<String, (Span, Lit)>);

impl Config {
    pub fn new(attribute_config: &AttributeConfig, main: Option<&str>) -> syn::Result<Self> {
        let mut map = BTreeMap::new();
        match attribute_config {
            AttributeConfig::None => Ok(Self(map)),
            AttributeConfig::Single(lit) => {
                if let Some(main) = main {
                    map.insert(main.to_string(), (lit.span(), lit.clone()));
                } else {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "no configuration is available",
                    ));
                };
                Ok(Self(map))
            }
            AttributeConfig::Multiple(attribute_params) => {
                for attribute_param in attribute_params {
                    let old = map.insert(
                        attribute_param.ident.to_string(),
                        (
                            attribute_param.ident.span(),
                            attribute_param.literal.clone(),
                        ),
                    );

                    if old.is_some() {
                        return Err(syn::Error::new_spanned(
                            &attribute_param.ident,
                            "duplicate config parameter",
                        ));
                    }
                }

                Ok(Self(map))
            }
        }
    }

    pub fn finish(self) -> syn::Result<()> {
        if let Some((ident, (span, _))) = self.0.into_iter().next() {
            return Err(syn::Error::new(
                span,
                format!("unknown config parameter `{}`", ident),
            ));
        } else {
            Ok(())
        }
    }
}

impl Config {
    pub fn get_lit(&mut self, ident: &str) -> Option<Lit> {
        self.0.remove(ident).map(|(_, lit)| lit)
    }

    pub fn get_lit_str(&mut self, ident: &'static str) -> syn::Result<Option<LitStr>> {
        match self.get_lit(ident) {
            Some(Lit::Str(lit_str)) => Ok(Some(lit_str)),
            None => Ok(None),
            Some(lit) => Err(syn::Error::new_spanned(lit, "expected string literal")),
        }
    }
}
