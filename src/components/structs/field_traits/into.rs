use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{config::Config, idents::config::CONFIG_DOC, order::OrderTrait, tokens::IndexedField};

const DEFAULT_DOC: &str = "Converts into the `{1}` field of [`{0}`].";

pub fn expand_into(
    input: &ItemStruct,
    indexed_field: &IndexedField<'_>,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &indexed_field.as_token().to_string()],
    )?;

    config.finish()?;

    let struct_ident = &input.ident;
    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("From", order.ident.span());
    let method_ident = Ident::new("from", order.ident.span());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::core::convert:: #trait_ident <#struct_ident #ty_generics> for #field_type #where_clause {
            #[doc = #doc]
            #[inline]
            fn #method_ident (value: #struct_ident #ty_generics) -> #field_type {
                value.#field_ident
            }
        }
    })
}
