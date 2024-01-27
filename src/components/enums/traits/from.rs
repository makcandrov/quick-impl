use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Fields;

use crate::attributes::{Attribute, AttributeConfig};
use crate::components::idents::DOC;
use crate::expand::Context;

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
                        DOC => config.set_doc(&param.literal)?,
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
    let (ty, destruct, ret) = match fields {
        Fields::Named(named_fields) => {
            let len = named_fields.named.len();

            let mut ty = TokenStream::default();
            let mut destruct = TokenStream::default();
            let mut ret = TokenStream::default();

            let mut i = 0;

            for field in &named_fields.named {
                let var_ident = field.ident.as_ref().unwrap();

                let field_ty = &field.ty;

                if i == len - 1 {
                    ty.extend(quote! { #field_ty });
                    destruct.extend(quote! { #var_ident });
                    ret.extend(quote! { #var_ident });
                } else {
                    ty.extend(quote! { #field_ty, });
                    destruct.extend(quote! { #var_ident, });
                    ret.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (
                if len == 1 {
                    quote! { #ty }
                } else {
                    quote! { ( #ty ) }
                },
                quote! { { #destruct } },
                if len == 1 {
                    quote! { #ret }
                } else {
                    quote! { ( #ret ) }
                },
            )
        },
        Fields::Unnamed(unnamed_fields) => {
            let len = unnamed_fields.unnamed.len();

            let mut ty = TokenStream::default();
            let mut destruct = TokenStream::default();
            let mut ret = TokenStream::default();

            let mut i = 0;

            for field in &unnamed_fields.unnamed {
                assert!(field.ident.is_none());

                let var_ident = Ident::new(&format!("arg{i}"), Span::call_site());

                let field_ty = &field.ty;

                if i == len - 1 {
                    ty.extend(quote! { #field_ty });
                    destruct.extend(quote! { #var_ident });
                    ret.extend(quote! { #var_ident });
                } else {
                    ty.extend(quote! { #field_ty, });
                    destruct.extend(quote! { #var_ident, });
                    ret.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (
                if len == 1 {
                    quote! { #ty }
                } else {
                    quote! { ( #ty ) }
                },
                quote! { ( #destruct ) },
                if len == 1 {
                    quote! { #ret }
                } else {
                    quote! { ( #ret ) }
                },
            )
        },
        Fields::Unit => (quote! { () }, quote! {}, quote! { () }),
    };

    let enum_ident = &context.ident;
    let (impl_generics, ty_generics, where_clause) = context.generics.split_for_impl();

    let doc = &config.doc;

    Ok(quote! {
        impl #impl_generics From<#ty> for #enum_ident #ty_generics #where_clause {
            #[doc = #doc]
            fn from(#ret: #ty) -> Self {
                Self::#variant_ident #destruct
            }
        }
    })
}
