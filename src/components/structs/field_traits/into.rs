use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::{
    attributes::Attribute, config::Config, expand::Context, idents::config::CONFIG_DOC,
    tokens::IndexedField,
};

const DEFAULT_DOC: &str = "Converts into the `{1}` field of [`{0}`].";

pub fn expand_into(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let mut config = Config::new(&attribute.config, None)?;

    let doc = config.get_formatted_lit_str(
        CONFIG_DOC,
        LitStr::new(DEFAULT_DOC, Span::call_site()),
        [
            &context.ident.to_string(),
            &indexed_field.as_token().to_string(),
        ],
    )?;

    config.finish()?;

    let struct_ident = &context.ident;
    let field_type = &indexed_field.ty;
    let field_ident = indexed_field.as_token();
    let trait_ident = Ident::new("From", attribute.ident.span());
    let method_ident = Ident::new("from", attribute.ident.span());

    let (impl_generics, ty_generics, where_clause) = context.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::core::convert:: #trait_ident <#struct_ident #ty_generics> for #field_type #where_clause {
            #[doc = #doc]
            #[inline]
            fn #method_ident (value: #struct_ident #ty_generics) -> #field_type {
                value.#field_ident
            }
        }
    })
}
