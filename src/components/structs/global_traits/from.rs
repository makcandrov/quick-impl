use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::{Fields, Ident};

use crate::{
    attributes::Attribute,
    config::Config,
    expand::Context,
    idents::config::CONFIG_DOC,
    tokens::{destructure_data, destructure_types, get_delimiter, with_delimiter, RenameField},
};

const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] from a tuple of its fields values.";

pub fn expand_from<'a>(
    context: &'a Context,
    attribute: &'a Attribute,
    fields: &'a Fields,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, None)?;

    let doc = config.get_lit_str_tokens(CONFIG_DOC)?.unwrap_or_else(|| {
        let doc = DEFAULT_DOC.replace("{}", &context.ident.to_string());
        quote! { #doc }
    });

    config.finish()?;

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

    let trait_ident = syn::Ident::new("From", attribute.ident.span());
    let method_ident = Ident::new("from", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (#ret: #ty) -> Self {
            Self #destruct
        }
    };

    Ok(context.in_impl(
        quote! { ::core::convert::#trait_ident<#ty> for },
        &content,
        None,
    ))
}
