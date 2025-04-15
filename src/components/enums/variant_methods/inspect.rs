use proc_macro2::{Delimiter, Span, TokenStream};
use quote::quote;
use syn::{LitStr, Variant};

use crate::{
    attributes::{Attribute, MethodAttribute},
    config::Config,
    expand::Context,
    idents::config::{CONFIG_DOC, CONFIG_NAME},
    tokens::{
        destructure_data, destructure_types, get_delimiter, with_delimiter, AloneDecoration,
        RenameField,
    },
    utils::to_snake_case,
};

const DEFAULT_NAME: &str = "inspect_{}";
const DEFAULT_DOC: &str = "Calls a function with a reference to the associated data if the instance is of the variant [`{}::{}`]. Returns the original instance.";

pub fn expand_inspect(
    context: &Context,
    variant: &Variant,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, Some(CONFIG_NAME))?;

    let method_ident = config.get_formatted_lit_str_ident(
        CONFIG_NAME,
        LitStr::new(DEFAULT_NAME, attribute.ident.span()),
        [&to_snake_case(&variant.ident.to_string())],
    )?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [&context.ident.to_string(), &variant.ident.to_string()],
    )?;

    config.finish()?;

    let fields = &variant.fields;
    let delimiter = get_delimiter(fields);

    let need_rename = fields
        .iter()
        .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "f"));

    let ty = destructure_types(
        fields,
        quote! { & },
        quote! { () },
        AloneDecoration::DelimitedNoComma,
    );

    let destruct = destructure_data(
        fields,
        quote! { ref },
        with_delimiter(TokenStream::new(), delimiter),
        delimiter,
        AloneDecoration::DelimitedNoComma,
        if need_rename {
            RenameField::Always
        } else {
            RenameField::Auto
        },
    );
    let args = destructure_data(
        fields,
        TokenStream::new(),
        quote! { () },
        Delimiter::Parenthesis,
        AloneDecoration::DelimitedNoComma,
        if need_rename {
            RenameField::AlwaysIgnoreOriginal
        } else {
            RenameField::Auto
        },
    );

    let variant_ident = &variant.ident;
    let keywords = method_attr.keywords();

    Ok(quote! {
        #[doc = #doc]
        #[inline]
        #keywords fn #method_ident(self, f: impl ::core::ops::FnOnce #ty) -> Self {
            if let Self:: #variant_ident #destruct = self {
                f #args;
            }
            self
        }
    })
}
