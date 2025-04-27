use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse2;

use crate::{
    attr::AllAttrs,
    components::{enum_impl, struct_impl},
    ctx::Context,
    input::Input,
    order::{AllOrders, Orders},
};

pub fn expand_qi(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    expand(args, input, false)
}

pub fn expand_qia(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    expand(args, input, true)
}

fn expand(args: TokenStream, input: TokenStream, all: bool) -> syn::Result<TokenStream> {
    let mut input = parse2::<Input>(input)?;

    let args_orders = parse2::<Orders>(args)?;
    let (global_from_args, all_from_args) =
        if all { (Default::default(), args_orders) } else { (args_orders, Default::default()) };

    // Extract all relevant attributes from the item definition and its variants or fields.
    let all_attrs = AllAttrs::extract_from_input(&mut input);

    // Parse all relevant attributes into orders.
    let all_orders = AllOrders::try_from_attrs(global_from_args, all_from_args, all_attrs)?;

    // All tokens generated from the input item will have this neutral span.
    // - Using `call_site` duplicates the number of implementations found by the analyzer.
    // - Picking a random token from the source attaches each analyzer annotation to that token.
    // No span I tested was free of weird side effects, so I chose the first option.
    let neutral_span = Span::call_site();

    // Manually remove all token spans so that the original spans never appear in the
    // implementations. The token stream with the original spans is returned to create the
    // structure with the correct spans.
    let (input, original_input_tokens) = input.respan(neutral_span);

    let mut implems = Implems::default();

    match &input {
        Input::Enum(input) => enum_impl(input, &mut implems, &all_orders)?,
        Input::Struct(input) => struct_impl(input, &mut implems, &all_orders)?,
    }

    let impls = implems.get_impls(&input);

    Ok(quote! {
        #original_input_tokens
        #impls
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

    fn get_impls(self, context: &impl Context) -> TokenStream {
        let methdos = if self.methods.is_empty() {
            if self.traits.is_empty() {
                return TokenStream::new();
            }

            TokenStream::new()
        } else {
            context.in_impl(Default::default(), &self.methods, None)
        };

        let mut traits = TokenStream::new();
        self.traits.into_iter().for_each(|tts| traits.extend(tts));

        quote! {
            #[allow(non_snake_case)]
            const _: () = {
                #methdos
                #traits
            };
        }
    }
}
