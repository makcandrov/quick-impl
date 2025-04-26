use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::{Fields, Ident, ItemStruct};

use crate::{
    attr::Attr,
    config::Config,
    idents::config::CONFIG_DOC,
    tokens::{
        AloneDecoration, RenameField, destructure_data, destructure_types, get_delimiter,
        with_delimiter,
    },
    utils::WithSpan,
};

const DEFAULT_DOC: &str = "Creates a tuple from each field of the instance of [`{}`].";

pub fn expand_into(
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
        TokenStream::new(),
        Delimiter::Parenthesis,
        AloneDecoration::None,
        RenameField::Auto,
    );

    let trait_ident = syn::Ident::new("From", attribute.ident.span());
    let method_ident = Ident::new("from", attribute.ident.span());

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident.clone().without_span();

    let mut result = quote! {
        impl #impl_generics ::core::convert:: #trait_ident <#ident #ty_generics> for #ty #where_clause {
            #[doc = #doc]
            #[inline]
            fn #method_ident (#ident #destruct: #ident #ty_generics) -> Self {
                #ret
            }
        }
    };

    // If there is exactly one field of type T, we need to implement both `Into<T>` and
    // `Into<(T,)>`.
    if fields.len() == 1 {
        let ty = quote! { (#ty,) };
        let ret = quote! { (#ret,) };

        result.extend(quote! {
            impl #impl_generics ::core::convert:: #trait_ident <#ident #ty_generics> for #ty #where_clause {
                #[doc = #doc]
                #[inline]
                fn #method_ident (#ident #destruct: #ident #ty_generics) -> Self {
                    #ret
                }
            }
        });
    }

    Ok(result)
}
