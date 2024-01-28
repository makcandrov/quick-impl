use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::Field;

use crate::attributes::{Attribute, AttributeConfig};
use crate::expand::Context;
use crate::idents::CONFIG_DOC;

struct Config {
    doc: String,
}

impl Config {
    pub fn default() -> Self {
        let doc = "Dereferences the value.".to_owned();

        Self { doc }
    }

    pub fn new(attribute: &Attribute) -> syn::Result<Self> {
        let mut config = Config::default();
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

pub fn struct_trait_deref(
    context: &Context,
    field: &Field,
    field_index: usize,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&attribute)?;

    let doc = &config.doc;

    let field_type = &field.ty;
    let field_ident = field
        .ident
        .clone()
        .unwrap_or_else(|| Ident::new(&field_index.to_string(), field.span()));
    let deref_trait = &attribute.ident;

    let content = quote! {
        type Target = #field_type;

        #[doc = #doc]
        fn deref(&self) -> &Self::Target {
            &self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { std::ops::#deref_trait for }, &content))
}
