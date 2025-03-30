use proc_macro2::{Delimiter, Ident, Span, TokenStream};
use quote::quote;
use syn::{LitStr, Variant};

use crate::{
    attributes::Attribute,
    config::Config,
    expand::Context,
    idents::config::CONFIG_DOC,
    tokens::{
        destructure_data, destructure_types, get_delimiter, with_delimiter, AloneDecoration,
        RenameField,
    },
};

const DEFAULT_DOC: &str = "Converts `self` into the associated data if the variant is [`{}::{}`]; otherwise, returns `Err(self)`.";

pub fn expand_try_into(
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

    let ty = destructure_types(
        fields,
        TokenStream::new(),
        quote! { () },
        AloneDecoration::None,
    );
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
    let trait_ident = Ident::new("TryInto", attribute.ident.span());
    let method_ident = Ident::new("try_into", attribute.ident.span());

    let content = quote! {
        type Error = Self;

        #[doc = #doc]
        #[inline]
        fn #method_ident (self) -> Result<#ty, Self> {
            match self {
                Self:: #variant_ident #destruct => Ok(#ret),
                other => Err(other),
            }
        }
    };

    Ok(context.in_impl(
        quote! { ::core::convert::#trait_ident<#ty> for },
        &content,
        None,
    ))
}
