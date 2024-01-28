use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::{Attribute, AttributeConfig};
use crate::expand::Context;
use crate::fields::IndexedField;
use crate::idents::config::CONFIG_DOC;

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
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&attribute)?;

    let doc = &config.doc;
    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let deref_trait = Ident::new("Deref", attribute.ident.span());
    let name = Ident::new("deref", attribute.ident.span());

    let content = quote! {
        type Target = #field_type;

        #[doc = #doc]
        fn #name (&self) -> &Self::Target {
            &self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { std::ops::#deref_trait for }, &content))
}
