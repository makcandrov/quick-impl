use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::{Attribute, AttributeConfig};
use crate::expand::Context;
use crate::fields::IndexedField;
use crate::idents::{CONFIG_DOC, TRAIT_DEREF_MUT};

struct Config {
    doc: String,
}

impl Config {
    pub fn default() -> Self {
        let doc = "Mutably dereferences the value.".to_owned();

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

pub fn struct_trait_deref_mut(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&attribute)?;

    let doc = &config.doc;
    let field_ident = indexed_field.as_token();
    let deref_trait = Ident::new(TRAIT_DEREF_MUT, attribute.ident.span());
    let name = Ident::new("deref_mut", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        fn #name (&mut self) -> &mut <Self as std::ops::Deref>::Target {
            &mut self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { std::ops::#deref_trait for }, &content))
}