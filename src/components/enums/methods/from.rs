use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
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

        let name = Ident::new(&format!("from_{variant_name_snake_case}"), attribute.ident.span());

        let doc = format!(
            "Generates a [`{}::{}`] variant from the associated data.",
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

pub fn enum_method_from(
    context: &Context,
    variant_ident: &Ident,
    fields: &Fields,
    attribute: &Attribute,
    visibility: &syn::Visibility,
    constant: bool,
) -> syn::Result<TokenStream> {
    let config = Config::new(&context.ident, variant_ident, &attribute)?;

    let (input, destruct) = match fields {
        Fields::Named(named_fields) => {
            let len = named_fields.named.len();

            let mut input = TokenStream::default();
            let mut destruct = TokenStream::default();

            let mut i = 0;

            for field in &named_fields.named {
                let var_ident = field.ident.as_ref().unwrap();

                let field_ty = &field.ty;

                if i == len - 1 {
                    input.extend(quote! { #var_ident: #field_ty });
                    destruct.extend(quote! { #var_ident });
                } else {
                    input.extend(quote! { #var_ident: #field_ty, });
                    destruct.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (quote! { #input }, quote! { { #destruct } })
        },
        Fields::Unnamed(unnamed_fields) => {
            let len = unnamed_fields.unnamed.len();

            let mut input = TokenStream::default();
            let mut destruct = TokenStream::default();

            let mut i = 0;

            for field in &unnamed_fields.unnamed {
                assert!(field.ident.is_none());

                let var_ident = Ident::new(&format!("arg{i}"), Span::call_site());

                let field_ty = &field.ty;

                if i == len - 1 {
                    input.extend(quote! { #var_ident: #field_ty });
                    destruct.extend(quote! { #var_ident });
                } else {
                    input.extend(quote! { #var_ident: #field_ty, });
                    destruct.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (quote! { #input }, quote! { ( #destruct ) })
        },
        Fields::Unit => (quote! {}, quote! {}),
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
        #vis #constant_kw fn #name(#input) -> Self {
            Self::#variant_ident #destruct
        }
    })
}
