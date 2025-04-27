use core::ops::{Deref, DerefMut};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, Lit, Meta, Token, Visibility, braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
};

use crate::attr::AllAttrs;

#[derive(Clone)]
pub enum Order {
    /// `pub const is`
    Method(OrderMethod),

    /// `impl From`
    Trait(OrderTrait),
}

#[derive(Clone)]
pub struct OrderMethod {
    pub vis: Visibility,
    pub const_token: Option<Token![const]>,
    pub ident: Ident,
    pub config: OrderConfigList,
}

#[derive(Clone)]
pub struct OrderTrait {
    #[expect(unused)]
    pub impl_token: Token![impl],
    pub ident: Ident,
    pub config: OrderConfigList,
}

#[derive(Clone)]
pub enum OrderConfigList {
    /// `pub const is`
    None,

    /// `pub const is = "is_bar"`
    Single(Lit),

    /// `pub const is = { name = "is_bar", doc = "Is this bar." }`
    Multiple {
        #[expect(unused)]
        brace: Brace,
        configs: Punctuated<OrderConfig, Token![,]>,
    },
}

#[derive(Clone)]
pub struct OrderConfig {
    pub ident: Ident,
    #[expect(unused)]
    pub eq_token: Token![=],
    pub literal: Lit,
}

impl OrderMethod {
    pub fn keywords(&self) -> TokenStream {
        let Self { vis, const_token, .. } = self;
        quote! { #vis #const_token }
    }
}

impl Parse for OrderConfig {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self { ident: input.parse()?, eq_token: input.parse()?, literal: input.parse()? })
    }
}

#[derive(Default, Clone)]
pub struct Orders(Vec<Order>);

impl Deref for Orders {
    type Target = Vec<Order>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Orders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Extend<Order> for Orders {
    fn extend<T: IntoIterator<Item = Order>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl IntoIterator for Orders {
    type Item = Order;
    type IntoIter = <Vec<Order> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Orders {
    fn try_from_meta(meta: &Meta) -> syn::Result<Self> {
        let Meta::List(list) = meta else {
            return Err(syn::Error::new_spanned(meta, "expected list of arguments"));
        };

        list.parse_args::<Self>()
    }
}

impl Parse for Order {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![impl]) {
            input.parse().map(Self::Trait)
        } else {
            input.parse().map(Self::Method)
        }
    }
}

impl Parse for OrderMethod {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            vis: input.parse()?,
            const_token: input.parse()?,
            ident: input.parse()?,
            config: input.parse()?,
        })
    }
}

impl Parse for OrderTrait {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self { impl_token: input.parse()?, ident: input.parse()?, config: input.parse()? })
    }
}

impl Parse for OrderConfigList {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![=]) {
            let _eq_token = input.parse::<Token![=]>()?;

            if input.peek(Lit) {
                input.parse().map(Self::Single)
            } else {
                let content;
                Ok(OrderConfigList::Multiple {
                    brace: braced!(content in input),
                    configs: content.call(Punctuated::parse_terminated)?,
                })
            }
        } else {
            Ok(OrderConfigList::None)
        }
    }
}

impl Parse for Orders {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content = Punctuated::<Order, Token![,]>::parse_terminated(input)?;
        Ok(Self(content.into_iter().collect()))
    }
}

#[derive(Clone, Default)]
pub struct AllOrders {
    /// Orders directly above the enum/struct
    global: Orders,

    /// Orders directly above the enum/struct inside of a `quick_impl_all` attribute
    all: Orders,

    /// Per variant/field attribues.
    /// The vector size _must_ match the variants/fields number.
    per_item: Vec<Orders>,
}

impl AllOrders {
    /// - `global_from_args`: [`Orders`] from the main [`#[quick_impl]`](macro@crate::quick_impl)
    ///   attribute, if any.
    /// - `all_from_args`: [`Orders`] from the main
    ///   [`#[quick_impl_all]`](macro@crate::quick_impl_all) attribute, if any.
    /// - `attrs`: All the other attributes, found in macro attribute helpers.
    pub fn try_from_attrs(
        global_from_args: Orders,
        all_from_args: Orders,
        attrs: AllAttrs,
    ) -> syn::Result<Self> {
        let mut all_orders = Self {
            global: global_from_args,
            all: all_from_args,
            per_item: Vec::with_capacity(attrs.per_item.len()),
        };

        for attr in attrs.global.quick_impl {
            all_orders.global.extend(Orders::try_from_meta(&attr.meta)?);
        }

        for attr in attrs.global.quick_impl_all {
            all_orders.all.extend(Orders::try_from_meta(&attr.meta)?);
        }

        for per_item in attrs.per_item {
            let mut orders = Orders::default();
            for attr in per_item.quick_impl {
                orders.extend(Orders::try_from_meta(&attr.meta)?);
            }
            all_orders.per_item.push(orders);

            if let Some(first) = per_item.quick_impl_all.first() {
                return Err(syn::Error::new_spanned(
                    first,
                    "`quick_impl_all` attribute is only allowed above the enum/struct definition",
                ));
            }
        }

        Ok(all_orders)
    }

    pub fn global(&self) -> impl Iterator<Item = &Order> {
        self.global.iter()
    }

    pub fn per_item(&self, i: usize) -> impl Iterator<Item = &Order> {
        self.all.iter().chain(self.per_item[i].iter())
    }
}
