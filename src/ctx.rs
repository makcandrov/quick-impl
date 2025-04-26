use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, ItemEnum, ItemStruct, WhereClause};

use crate::{expand::Input, utils::WithSpan};

pub trait Context {
    fn generics(&self) -> &Generics;
    fn ident(&self) -> &Ident;

    fn in_impl(
        &self,
        trait_for: TokenStream,
        tokens: &TokenStream,
        additional_where_clause: Option<WhereClause>,
    ) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let where_clause = match (where_clause.cloned(), additional_where_clause) {
            (None, None) => None,
            (None, Some(where_clause)) => Some(where_clause),
            (where_clause @ Some(_), None) => where_clause,
            (Some(mut where_clause), Some(additional_where_clause)) => {
                where_clause.predicates.extend(additional_where_clause.predicates);
                Some(where_clause)
            }
        };
        let ident = self.ident().clone().without_span();
        quote! {
            impl #impl_generics #trait_for #ident #ty_generics #where_clause {
                #tokens
            }
        }
    }
}

impl Context for ItemStruct {
    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Context for ItemEnum {
    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Context for Input {
    fn generics(&self) -> &Generics {
        match self {
            Input::Struct(item) => Context::generics(item),
            Input::Enum(item) => Context::generics(item),
        }
    }

    fn ident(&self) -> &Ident {
        match self {
            Input::Struct(item) => Context::ident(item),
            Input::Enum(item) => Context::ident(item),
        }
    }
}
