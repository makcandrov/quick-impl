use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::attributes::Attribute;
use crate::config::{build_config, build_enum_doc};
use crate::expand::Context;
use crate::tokens::IndexedField;

build_enum_doc! {
    ConfigDoc,
    "Mutably dereferences the value.{0:.0}{1:.0}",
}

build_config! {
    Config,
    (doc, ConfigDoc, false),
}

pub fn struct_trait_deref_mut(
    context: &Context,
    indexed_field: &IndexedField<'_>,
    attribute: &Attribute,
) -> syn::Result<TokenStream> {
    let config = Config::new(context, attribute, indexed_field)?;

    let doc = &config.doc;
    let field_ident = indexed_field.as_token();
    let deref_mut_trait = Ident::new("DerefMut", attribute.ident.span());
    let name = Ident::new("deref_mut", attribute.ident.span());

    let content = quote! {
        #[doc = #doc]
        fn #name (&mut self) -> &mut <Self as ::core::ops::Deref>::Target {
            &mut self.#field_ident
        }
    };

    Ok(context.in_impl(quote! { ::core::ops::#deref_mut_trait for }, &content))
}
