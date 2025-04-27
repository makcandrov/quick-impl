use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    Attribute, ItemEnum, ItemStruct, Token, Visibility,
    parse::{Parse, ParseStream},
    parse2,
};

use crate::respan::respan2;

#[derive(Clone)]
pub enum Input {
    Enum(ItemEnum),
    Struct(ItemStruct),
}

impl Input {
    pub fn respan(self, span: Span) -> (Self, TokenStream) {
        let spanned_tokens = self.into_token_stream();
        let respanned_tokens = respan2(spanned_tokens.clone(), span);

        // Cannot fail as it originates from an `Input` tokens.
        let respanned_self = parse2(respanned_tokens).unwrap();
        (respanned_self, spanned_tokens)
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
