use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, Lit, LitStr};

use crate::{
    order::OrderConfigList,
    utils::{runtime_format, set_lit_str_value},
};

#[must_use]
pub struct Config(BTreeMap<String, (Span, Lit)>);

impl Config {
    pub fn new(order_config_list: &OrderConfigList, main: Option<&str>) -> syn::Result<Self> {
        let mut map = BTreeMap::new();
        match order_config_list {
            OrderConfigList::None => Ok(Self(map)),
            OrderConfigList::Single(lit) => {
                if let Some(main) = main {
                    map.insert(main.to_string(), (lit.span(), lit.clone()));
                } else {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "no default configuration is available",
                    ));
                };
                Ok(Self(map))
            }
            OrderConfigList::Multiple { brace: _, configs } => {
                for config in configs {
                    let old = map.insert(
                        config.ident.to_string(),
                        (config.ident.span(), config.literal.clone()),
                    );

                    if old.is_some() {
                        return Err(syn::Error::new_spanned(
                            &config.ident,
                            format!("duplicate config parameter `{}`", &config.ident),
                        ));
                    }
                }

                Ok(Self(map))
            }
        }
    }

    pub fn finish(self) -> syn::Result<()> {
        if let Some((ident, (span, _))) = self.0.into_iter().next() {
            Err(syn::Error::new(span, format!("unknown config parameter `{}`", ident)))
        } else {
            Ok(())
        }
    }
}

impl Config {
    pub fn get_lit(&mut self, config_ident: &str) -> Option<Lit> {
        self.0.remove(config_ident).map(|(_, lit)| lit)
    }

    pub fn get_lit_str(&mut self, config_ident: &'static str) -> syn::Result<Option<LitStr>> {
        match self.get_lit(config_ident) {
            Some(Lit::Str(lit_str)) => Ok(Some(lit_str)),
            None => Ok(None),
            Some(lit) => Err(syn::Error::new_spanned(lit, "expected string literal")),
        }
    }

    pub fn get_lit_str_tokens(
        &mut self,
        config_ident: &'static str,
    ) -> syn::Result<Option<TokenStream>> {
        let lit_str = self.get_lit_str(config_ident)?;
        Ok(lit_str.map(|lit_str| {
            let value = lit_str.value();
            let span = lit_str.span();
            quote_spanned! {span=> #value}
        }))
    }

    pub fn get_lit_str_ident(&mut self, config_ident: &'static str) -> syn::Result<Option<Ident>> {
        let lit_str = self.get_lit_str(config_ident)?;
        Ok(lit_str.map(|lit_str| {
            let value = lit_str.value();
            let span = lit_str.span();
            Ident::new(&value, span)
        }))
    }

    pub fn get_formatted_lit_str(
        &mut self,
        config_ident: &'static str,
        default_lit_str: LitStr,
        replacements: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> syn::Result<LitStr> {
        let mut lit_str = self.get_lit_str(config_ident)?.unwrap_or(default_lit_str);
        let value = runtime_format(lit_str.value(), replacements);
        set_lit_str_value(&mut lit_str, value);
        Ok(lit_str)
    }

    pub fn get_formatted_lit_str_ident(
        &mut self,
        config_ident: &'static str,
        default_lit_str: LitStr,
        replacements: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> syn::Result<Ident> {
        let lit_str = self.get_formatted_lit_str(config_ident, default_lit_str, replacements)?;
        let value = lit_str.value();
        let span = lit_str.span();
        Ok(Ident::new(&value, span))
    }
}
