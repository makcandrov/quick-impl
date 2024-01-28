use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::{Attribute, AttributeConfig, MethodAttribute};
use crate::expand::Context;
use crate::fields::IndexedField;
use crate::idents::{CONFIG_DOC, CONFIG_NAME, TRAIT_DEREF_MUT};

struct Config {
    name: syn::Ident,
    doc: String,
}

impl Config {
    pub fn default(
        struct_ident: &syn::Ident,
        indexed_field: &IndexedField<'_>,
        attribute: &Attribute,
    ) -> syn::Result<Self> {
        let Some(field_ident) = &indexed_field.ident else {
            return Err(syn::Error::new_spanned(
                &attribute.ident,
                "Cannot infer getter name of unnamed field",
            ));
        };

        let field_name_snake_case = field_ident.to_string().to_case(Case::Snake);

        let name = Ident::new(&format!("get_{field_name_snake_case}"), attribute.ident.span());

        let doc = format!("A getter for the field `{}` of [{}].", field_ident, struct_ident,);

        Ok(Self { name, doc })
    }

    pub fn new(
        struct_ident: &syn::Ident,
        indexed_field: &IndexedField<'_>,
        attribute: &Attribute,
    ) -> syn::Result<Self> {
        let mut config = Config::default(struct_ident, indexed_field, attribute)?;
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

pub fn struct_method_get(
    context: &Context,
    indexed_field: &IndexedField,
    attribute: &Attribute,
    method_attr: &MethodAttribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(&context.ident, indexed_field, attribute)?;

    let doc = &config.doc;

    let ty = &indexed_field.ty;
    let keywords = method_attr.keywords();
    let name = &config.name;
    let field_ident = indexed_field.as_token();

    Ok(quote! {
        #[doc = #doc]
        #keywords fn #name (&self) -> & #ty {
            &self.#field_ident
        }
    })
}
