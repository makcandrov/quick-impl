use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, ItemEnum, ItemStruct, Token, Visibility,
    parse::{Parse, ParseStream},
    parse2,
};

use crate::{
    attr::Attrs,
    components::{enum_impl, struct_impl},
    ctx::Context,
};

pub fn expand_qi(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    expand::<false>(args, input)
}

pub fn expand_qia(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    expand::<true>(args, input)
}

#[derive(Clone)]
pub enum Input {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

impl Input {
    pub fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Input::Struct(item) => &mut item.attrs,
            Input::Enum(item) => &mut item.attrs,
        }
    }
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let fork = input.fork();
        let _ = fork.call(Attribute::parse_outer)?;
        let _ = fork.parse::<Visibility>()?;

        if fork.peek(Token![struct]) {
            input.parse().map(Self::Struct)
        } else if fork.peek(Token![enum]) {
            input.parse().map(Self::Enum)
        } else {
            Err(fork.error("expected a struct or an enum"))
        }
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Input::Struct(item) => item.to_tokens(tokens),
            Input::Enum(item) => item.to_tokens(tokens),
        }
    }
}

fn expand<const ALL: bool>(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args_attrs = parse2::<Attrs>(args)?;

    let (mut all_attrs, mut glob_attrs) =
        if ALL { (args_attrs, Default::default()) } else { (Default::default(), args_attrs) };

    let mut input = parse2::<Input>(input)?;
    let mut implems = Implems::default();

    all_attrs.extend(Attrs::take_from(input.attrs_mut(), true)?);
    glob_attrs.extend(Attrs::take_from(input.attrs_mut(), false)?);

    match &mut input {
        Input::Struct(item) => struct_impl(item, &mut implems, &all_attrs, &glob_attrs)?,
        Input::Enum(item) => enum_impl(item, &mut implems, &all_attrs, &glob_attrs)?,
    }

    let methods = implems.get_methods(&input);
    let traits = implems.get_traits();

    Ok(quote! {
        #input
        #methods
        #traits
    })
}

#[derive(Debug, Clone, Default)]
pub struct Implems {
    methods: TokenStream,
    traits: Vec<TokenStream>,
}

impl Implems {
    pub fn extend_methods(&mut self, tokens: TokenStream) {
        self.methods.extend(tokens)
    }

    pub fn extend_traits(&mut self, tokens: TokenStream) {
        self.traits.push(tokens)
    }

    pub fn get_methods(&self, context: &impl Context) -> TokenStream {
        if self.methods.is_empty() {
            TokenStream::new()
        } else {
            let methods_impl = context.in_impl(Default::default(), &self.methods, None);
            quote! {
                #[allow(non_snake_case)]
                #methods_impl
            }
        }
    }

    pub fn get_traits(&self) -> TokenStream {
        let mut traits_impl = TokenStream::new();

        if self.traits.is_empty() {
            return traits_impl;
        }

        for t in &self.traits {
            traits_impl.extend(quote! { #t })
        }
        quote! {
            #[allow(non_snake_case)]
            const _: () = {
                #traits_impl
            };
        }
    }
}
