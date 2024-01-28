use std::ops::{Deref, DerefMut};

use syn::parse::Parse;

use crate::idents::MACRO_PATH;

#[derive(Clone)]
pub struct Attribute {
    pub ident: syn::Ident,
    pub typ: AttributeType,
    pub config: AttributeConfig,
}

#[derive(Clone)]
pub enum AttributeType {
    Method {
        visibility: syn::Visibility,
        constant: bool,
    },
    Trait,
}

#[derive(Clone)]
pub enum AttributeConfig {
    None,
    Single(syn::Lit),
    Multiple(Vec<AttributeParam>),
}

#[derive(Clone)]
pub struct AttributeParam {
    pub ident: syn::Ident,
    pub literal: syn::Lit,
}

#[derive(Default)]
pub struct Attributes(Vec<Attribute>);

impl Deref for Attributes {
    type Target = Vec<Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Attributes {
    pub fn from_attributes(attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut res = Self::default();
        for attr in attrs {
            res.extend(Self::from_attribute(attr)?.0);
        }
        Ok(res)
    }

    pub fn from_attribute(attr: &syn::Attribute) -> syn::Result<Self> {
        if !attr.path().is_ident(MACRO_PATH) {
            return Ok(Self::default());
        }

        let syn::Meta::List(list) = &attr.meta else {
            return Err(syn::Error::new_spanned(
                attr,
                &format!("Expected `{MACRO_PATH}(...)` attribute."),
            ));
        };

        list.parse_args::<Self>()
    }
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let typ = if lookahead.peek(syn::token::Impl) {
            input.parse::<syn::token::Impl>().unwrap();
            AttributeType::Trait
        } else {
            let visibility = input.parse::<syn::Visibility>()?;

            let lookahead = input.lookahead1();
            let constant = if lookahead.peek(syn::token::Const) {
                input.parse::<syn::token::Const>().unwrap();
                true
            } else {
                false
            };

            AttributeType::Method { visibility, constant }
        };

        let ident = input.parse::<syn::Ident>()?;

        let lookahead = input.lookahead1();
        let config = if lookahead.peek(syn::token::Eq) {
            input.parse::<syn::token::Eq>().unwrap();

            if let Ok(lit) = input.parse::<syn::Lit>() {
                AttributeConfig::Single(lit)
            } else {
                let braced;
                syn::braced!(braced in input);

                let content =
                    syn::punctuated::Punctuated::<AttributeParam, syn::token::Comma>::parse_terminated(&braced)?;

                let config = content.into_iter().collect::<Vec<_>>();

                AttributeConfig::Multiple(config)
            }
        } else {
            AttributeConfig::None
        };

        Ok(Attribute { typ, ident, config })
    }
}

impl Parse for AttributeParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        input.parse::<syn::token::Eq>()?;
        let literal = input.parse::<syn::Lit>()?;

        Ok(Self { ident, literal })
    }
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = syn::punctuated::Punctuated::<Attribute, syn::token::Comma>::parse_terminated(input)?;
        Ok(Self(content.into_iter().collect()))
    }
}
