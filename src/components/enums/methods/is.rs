use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::Fields;

use crate::attributes::{Attribute, AttributeConfig};
use crate::components::idents::{DOC, NAME};
use crate::expand::Context;

struct Config {
    name: syn::Ident,
    doc: String,
}

impl Config {
    pub fn default(enum_ident: &syn::Ident, variant_ident: &syn::Ident, attribute: &Attribute) -> Self {
        let variant_name_snake_case = variant_ident.to_string().to_case(Case::Snake);

        let name = Ident::new(&format!("is_{variant_name_snake_case}"), attribute.ident.span());

        let doc = format!(
            "Returns `true` if it is the [`{}::{}`] variant. Otherwise, returns `false`.",
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
                        NAME => config.set_name(&param.literal)?,
                        DOC => config.set_doc(&param.literal)?,
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

pub fn enum_method_is(
    context: &Context,
    variant_ident: &Ident,
    fields: &Fields,
    attribute: &Attribute,
    visibility: &syn::Visibility,
    constant: bool,
) -> syn::Result<TokenStream> {
    let config = Config::new(&context.ident, variant_ident, &attribute)?;

    let destruct = match fields {
        Fields::Named(_) => quote! { { .. } },
        Fields::Unnamed(_) => quote! { ( .. ) },
        Fields::Unit => quote! {},
    };

    let vis = visibility.to_token_stream();
    let constant_kw = if constant {
        quote! { const }
    } else {
        quote! {}
    };
    let doc = config.doc;
    let name = config.name;

    Ok(quote! {
        #[doc = #doc]
        #vis #constant_kw fn #name(&self) -> bool {
            match self {
                Self::#variant_ident #destruct => true,
                _ => false,
            }
        }
    })
}
