use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::components::{enum_impl, struct_impl};
use crate::idents::MACRO_PATH;

pub fn derive(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(expanded) => expanded,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #error
            }
        },
    }
}

pub struct Context<'a> {
    pub ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        Self {
            ident: &input.ident,
            generics: &input.generics,
        }
    }

    pub fn in_impl(&self, trait_for: TokenStream, tokens: &TokenStream) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let ident = self.ident;
        quote! {
            impl #impl_generics #trait_for #ident #ty_generics #where_clause {
                #tokens
            }
        }
    }
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

    pub fn get_methods(&self, context: &Context) -> TokenStream {
        context.in_impl(Default::default(), &self.methods)
    }

    pub fn get_traits(&self) -> TokenStream {
        let mut res = TokenStream::new();
        for t in &self.traits {
            res.extend(quote! { #t })
        }
        res
    }
}

fn try_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let context = Context::new(input);
    let mut implems = Implems::default();

    if let Some(attr) = input.attrs.iter().find(|attr| attr.path().is_ident(MACRO_PATH)) {
        return Err(syn::Error::new_spanned(attr, "Global attributes unavailable."));
    }

    match &input.data {
        Data::Struct(data_struct) => struct_impl(&context, &mut implems, data_struct)?,
        Data::Enum(data_enum) => enum_impl(&context, &mut implems, data_enum)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(input, "Unions are not supported")),
    }

    let methods = implems.get_methods(&context);
    let traits = implems.get_traits();

    Ok(quote! {
        #methods
        #traits
    })
}
