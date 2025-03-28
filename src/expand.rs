use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, WhereClause};

use crate::{
    attributes::Attributes,
    components::{enum_impl, struct_impl},
};

pub fn derive(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(expanded) => expanded,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #error
            }
        }
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

    pub fn in_impl(
        &self,
        trait_for: TokenStream,
        tokens: &TokenStream,
        additional_where_clause: Option<WhereClause>,
    ) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let where_clause = match (where_clause.cloned(), additional_where_clause) {
            (None, None) => None,
            (None, Some(where_clause)) => Some(where_clause),
            (where_clause @ Some(_), None) => where_clause,
            (Some(mut where_clause), Some(additional_where_clause)) => {
                where_clause
                    .predicates
                    .extend(additional_where_clause.predicates);
                Some(where_clause)
            }
        };
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

fn try_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let context = Context::new(input);
    let mut implems = Implems::default();

    let global_attributes = Attributes::from_attributes(&input.attrs)?;

    match &input.data {
        Data::Struct(data_struct) => {
            struct_impl(&context, &mut implems, &global_attributes, data_struct)?
        }
        Data::Enum(data_enum) => enum_impl(&context, &mut implems, &global_attributes, data_enum)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(input, "Unions are not supported")),
    }

    let methods = implems.get_methods(&context);
    let traits = implems.get_traits();

    Ok(quote! {
        #methods
        #traits
    })
}
