use convert_case::{Case, Casing};
use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::quote;
use syn::Fields;

use crate::attributes::{Attribute, AttributeConfig, MethodAttribute};
use crate::expand::Context;
use crate::fields::{destructure_data, destructure_types, get_delimiter};
use crate::idents::{CONFIG_DOC, CONFIG_NAME};

struct Config {
    name: syn::Ident,
    doc: String,
}

impl Config {
    pub fn default(enum_ident: &syn::Ident, variant_ident: &syn::Ident, attribute: &Attribute) -> Self {
        let variant_name_snake_case = variant_ident.to_string().to_case(Case::Snake);

        let name = Ident::new(&format!("into_{variant_name_snake_case}"), attribute.ident.span());

        let doc = format!(
            "Converts into the associated data if it is the [`{}::{}`] variant. Otherwise, returns `None`.",
            enum_ident, variant_ident
        );

        Self { name, doc }
    }

    pub fn new(enum_ident: &syn::Ident, variant_ident: &syn::Ident, attribute: &Attribute) -> syn::Result<Self> {
        let mut config = Config::default(enum_ident, variant_ident, attribute);
        match &attribute.config {
            AttributeConfig::None => {},
            AttributeConfig::Single(lit) => config.set_name(lit)?,
            AttributeConfig::Multiple(params) => {
                for param in params {
                    match param.ident.to_string().as_str() {
                        CONFIG_NAME => config.set_name(&param.literal)?,
                        CONFIG_DOC => config.set_doc(&param.literal)?,
                        _ => return Err(syn::Error::new_spanned(&param.ident, "Unknown parameter.")),
                    }
                }
            },
        }
        Ok(config)
    }

    pub fn set_name(&mut self, lit: &syn::Lit) -> syn::Result<()> {
        let syn::Lit::Str(lit_str) = lit else {
            return Err(syn::Error::new_spanned(lit, "Expected string literal."));
        };
        self.name = Ident::new(&lit_str.value(), lit_str.span());
        Ok(())
    }

    pub fn set_doc(&mut self, lit: &syn::Lit) -> syn::Result<()> {
        let syn::Lit::Str(lit_str) = lit else {
            return Err(syn::Error::new_spanned(lit, "Expected string literal."));
        };
        self.doc = lit_str.value();
        Ok(())
    }
}

pub fn enum_method_into(
    context: &Context,
    variant_ident: &Ident,
    fields: &Fields,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&context.ident, variant_ident, &attribute)?;

    let delimiter = get_delimiter(fields);

    let ty = destructure_types(fields, quote! {}, quote! { () }, true);
    let destruct = destructure_data(fields, quote! {}, delimiter, quote! {}, true);
    let ret = destructure_data(fields, quote! {}, Delimiter::Parenthesis, quote! { () }, false);

    let keywords = method_attr.keywords();

    let doc = config.doc;
    let name = config.name;

    Ok(quote! {
        #[doc = #doc]
        #keywords fn #name(self) -> Option<#ty> {
            match self {
                Self::#variant_ident #destruct => Some(#ret),
                _ => None,
            }
        }
    })
}
