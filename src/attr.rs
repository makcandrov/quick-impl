use core::ops::{Deref, DerefMut};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, Lit, Meta, Token, Visibility,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::{
    idents::{HELPER_QUICK_IMPL, HELPER_QUICK_IMPL_ALL},
    utils::{ThenTry, TryRetain},
};

#[derive(Clone)]
pub struct Attr {
    pub ident: Ident,
    pub kind: AttrKind,
    pub config: AttrConfigList,
}

#[derive(Clone)]
pub enum AttrKind {
    /// `pub const is`
    Method(AttrMethod),

    /// `impl From`
    Trait,
}

#[derive(Clone)]
pub struct AttrMethod {
    pub vis: Visibility,
    pub const_token: Option<Token![const]>,
}

impl AttrMethod {
    pub fn keywords(&self) -> TokenStream {
        let Self { vis, const_token } = self;
        quote! { #vis #const_token }
    }
}

#[derive(Clone)]
pub enum AttrConfigList {
    /// `pub const is`
    None,

    /// `pub const is = "is_bar"`
    Single(Lit),

    /// `pub const is = { name = "is_bar", doc = "Is this bar." }`
    Multiple(Vec<AttrConfig>),
}

#[derive(Clone)]
pub struct AttrConfig {
    pub ident: Ident,
    #[expect(unused)]
    pub eq_token: Token![=],
    pub literal: Lit,
}

impl Parse for AttrConfig {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self { ident: input.parse()?, eq_token: input.parse()?, literal: input.parse()? })
    }
}

#[derive(Default)]
pub struct Attrs(Vec<Attr>);

impl Deref for Attrs {
    type Target = Vec<Attr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Attrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Extend<Attr> for Attrs {
    fn extend<T: IntoIterator<Item = Attr>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl IntoIterator for Attrs {
    type Item = Attr;
    type IntoIter = <Vec<Attr> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Attrs {
    pub fn take_from(attrs: &mut Vec<Attribute>, all: bool) -> syn::Result<Self> {
        let mut res = Self(Vec::with_capacity(attrs.len()));
        let helper = if all { HELPER_QUICK_IMPL_ALL } else { HELPER_QUICK_IMPL };

        attrs.try_retain(|attr| -> Result<bool, syn::Error> {
            if !attr.path().is_ident(helper) {
                return Ok(true);
            }

            let new_attributes = Self::try_from_meta(&attr.meta)?;
            res.extend(new_attributes);

            Ok(false)
        })?;
        Ok(res)
    }

    fn try_from_meta(meta: &Meta) -> syn::Result<Self> {
        let Meta::List(list) = meta else {
            return Err(syn::Error::new_spanned(meta, "expected list of arguments"));
        };

        list.parse_args::<Self>()
    }
}

impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kind = if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            AttrKind::Trait
        } else {
            let vis = input.parse::<syn::Visibility>()?;
            let const_token = input.peek(Token![const]).then_try(|| input.parse())?;

            AttrKind::Method(AttrMethod { vis, const_token })
        };

        let ident = input.parse::<Ident>()?;

        let config = if input.peek(Token![=]) {
            input.parse::<Token![=]>().unwrap();

            if let Ok(lit) = input.parse::<syn::Lit>() {
                AttrConfigList::Single(lit)
            } else {
                let braced;
                syn::braced!(braced in input);

                let content = Punctuated::<AttrConfig, Token![,]>::parse_terminated(&braced)?;
                let config = content.into_iter().collect::<Vec<_>>();

                AttrConfigList::Multiple(config)
            }
        } else {
            AttrConfigList::None
        };

        Ok(Attr { kind, ident, config })
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content = Punctuated::<Attr, Token![,]>::parse_terminated(input)?;
        Ok(Self(content.into_iter().collect()))
    }
}
