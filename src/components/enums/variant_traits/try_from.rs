use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::quote;
use syn::{LitStr, Variant};

use crate::{
    attributes::Attribute,
    config::Config,
    expand::Context,
    idents::config::CONFIG_DOC,
    tokens::{destructure_data, destructure_types, get_delimiter, with_delimiter, RenameField},
};

const DEFAULT_DOC: &str = "Converts the instance into the associated data if the variant is [`{}::{}`]; otherwise, returns `Err(self)`.";

pub fn expand_try_from(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&context.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, TokenStream::new(), quote! { () }, false);
    let destruct = destructure_data(
        fields,
        TokenStream::new(),
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        true,
        RenameField::Auto,
    );
    let ret = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        false,
        RenameField::Auto,
    );

    let variant_ident = &variant.ident;
    let trait_ident = syn::Ident::new("TryFrom", attribute.ident.span());
    let method_ident = Ident::new("try_from", attribute.ident.span());

    let (impl_generics, ty_generics, where_clause) = context.generics.split_for_impl();
    let ident = context.ident;

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
