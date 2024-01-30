use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::Fields;

use crate::attributes::{Attribute, AttributeConfig};
use crate::expand::Context;
use crate::fields::{destructure_data, destructure_types, get_delimiter};
use crate::idents::config::CONFIG_DOC;

struct Config {
    doc: String,
}

impl Config {
    pub fn default(enum_ident: &syn::Ident, variant_ident: &syn::Ident) -> Self {
        let doc = format!(
            "Creates a [`{}::{}`] variant from the associated data.",
            enum_ident, variant_ident
        );

        Self { doc }
    }

    pub fn new(enum_ident: &syn::Ident, variant_ident: &syn::Ident, attribute: &Attribute) -> syn::Result<Self> {
        let mut config = Config::default(enum_ident, variant_ident);
        match &attribute.config {
            AttributeConfig::None => {},
            AttributeConfig::Single(lit) => return Err(syn::Error::new_spanned(lit, "Unexpected literal.")),
            AttributeConfig::Multiple(params) => {
                for param in params {
                    match param.ident.to_string().as_str() {
                        CONFIG_DOC => config.set_doc(&param.literal)?,
                        _ => return Err(syn::Error::new_spanned(&param.ident, "Unknown parameter.")),
                    }
                }
            },
        }
        Ok(config)
    }

    pub fn set_doc(&mut self, lit: &syn::Lit) -> syn::Result<()> {
        let syn::Lit::Str(lit_str) = lit else {
            return Err(syn::Error::new_spanned(lit, "Expected string literal."));
        };
        self.doc = lit_str.value();
        Ok(())
    }
}

pub fn enum_trait_from(
    context: &Context,
    variant_ident: &Ident,
    fields: &Fields,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&context.ident, variant_ident, &attribute)?;

    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, quote! {}, quote! { () }, false);
    let destruct = destructure_data(fields, quote! {}, quote! {}, delimiter, true);
    let ret = destructure_data(fields, quote! {}, quote! { () }, Delimiter::Parenthesis, false);

    let doc = &config.doc;
    let from_trait = syn::Ident::new("From", attribute.ident.span());
    let name = Ident::new("from", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        fn #name (#ret: #ty) -> Self {
            Self::#variant_ident #destruct
        }
    };

    Ok(context.in_impl(quote! { #from_trait<#ty> for }, &content))
}
