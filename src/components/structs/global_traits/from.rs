use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::{Fields, Ident, ItemStruct};

use crate::{
    attr::Attr,
    config::Config,
    ctx::Context,
    idents::config::CONFIG_DOC,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_types, get_delimiter,
        with_delimiter,
    },
};

const DEFAULT_DOC: &str = "Constructs a new instance of [`{}`] from a tuple of its fields values.";

pub fn expand_from(
    input: &ItemStruct,
    attribute: &Attr,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, None)?;

    let doc = config.get_lit_str_tokens(CONFIG_DOC)?.unwrap_or_else(|| {
        let doc = DEFAULT_DOC.replace("{}", &input.ident.to_string());
        quote! { #doc }
    });

    config.finish()?;

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
    let arg = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::None,
        RenameField::Auto,
    );

    let trait_ident = Ident::new("From", attribute.ident.span());
    let method_ident = Ident::new("from", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        #[inline]
        fn #method_ident (#arg: #ty) -> Self {
            Self #destruct
        }
    };

    let mut result = input.in_impl(
        quote! { ::core::convert::#trait_ident<#ty> for },
        &content,
        None,
    );

    // If there is exactly one field of type T, we need to implement both `From<T>` and `From<(T,)>`.
    if fields.len() == 1 {
        let ty = quote! { (#ty,) };
        let arg = quote! { (#arg,) };

        let content = quote! {
            #[doc = #doc]
            #[inline]
            fn #method_ident (#arg: #ty) -> Self {
                Self #destruct
            }
        };

        result.extend(input.in_impl(
            quote! { ::core::convert::#trait_ident<#ty> for },
            &content,
            None,
        ));
    }

    Ok(result)
}
