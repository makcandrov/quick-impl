use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemEnum, LitStr, Variant};

use crate::{
    config::Config,
    idents::config::CONFIG_DOC,
    order::OrderTrait,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_types, get_delimiter,
        with_delimiter,
    },
};

const DEFAULT_DOC: &str = "Converts the instance into the associated data if the variant is [`{}::{}`]; otherwise, returns `Err(self)`.";

pub fn expand_try_from(
    input: &ItemEnum,
    variant: &Variant,
    order: &OrderTrait,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&order.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&input.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, TokenStream::new(), quote! { () }, AloneDecoration::None);
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        RenameField::Auto,
    );
    let ret = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::None,
        RenameField::Auto,
    );

    let variant_ident = &variant.ident;
    let trait_ident = Ident::new("TryFrom", order.ident.span());
    let method_ident = Ident::new("try_from", order.ident.span());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;

    let content = quote! {
        impl #impl_generics ::core::convert:: #trait_ident <#ident #ty_generics> for #ty #where_clause {
            type Error = #ident #ty_generics;

            #[doc = #doc]
            #[inline]
            fn #method_ident (value: #ident #ty_generics) -> Result<Self, #ident #ty_generics> {
                match value {
                    #ident :: #variant_ident #destruct => Ok(#ret),
                    other => Err(other),
                }
            }
        }
    };

    Ok(content)
}
