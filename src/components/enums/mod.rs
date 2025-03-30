use syn::DataEnum;

use crate::{
    attributes::{AttributeType, Attributes},
    expand::{Context, Implems},
    idents::{methods::*, traits::*},
};

mod variant_methods;
mod variant_traits;

pub fn enum_impl(
    context: &Context<'_>,
    implems: &mut Implems,
    global_attributes: &Attributes,
    data_enum: &DataEnum,
) -> syn::Result<()> {
    for attribute in global_attributes.iter() {
        match &attribute.typ {
            AttributeType::Method(_) => {
                return Err(syn::Error::new_spanned(
                    &attribute.ident,
                    "invalid method name",
                ));
            }
            AttributeType::Trait => {
                return Err(syn::Error::new_spanned(
                    &attribute.ident,
                    "invalid trait name",
                ));
            }
        }
    }

    for variant in &data_enum.variants {
        let variant_attributes = Attributes::from_attributes(&variant.attrs)?;

        for attribute in variant_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => variant_methods::expand_as_ref_mut(
                            context,
                            &variant,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_AS_REF => variant_methods::expand_as_ref(
                            context,
                            &variant,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_FROM => {
                            variant_methods::expand_from(context, &variant, attribute, method_attr)?
                        }
                        METHOD_INSPECT => variant_methods::expand_inspect(
                            context,
                            variant,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_INTO => {
                            variant_methods::expand_into(context, &variant, attribute, method_attr)?
                        }
                        METHOD_IS_AND => variant_methods::expand_is_and(
                            context,
                            &variant,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_IS => {
                            variant_methods::expand_is(context, &variant, attribute, method_attr)?
                        }
                        METHOD_SET => {
                            variant_methods::expand_set(context, &variant, attribute, method_attr)?
                        }
                        METHOD_TRY_INTO => variant_methods::expand_try_into(
                            context,
                            &variant,
                            attribute,
                            method_attr,
                        )?,
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &attribute.ident,
                                "invalid method name",
                            ));
                        }
                    };
                    implems.extend_methods(tokens);
                }
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_FROM => variant_traits::expand_from(context, &variant, attribute)?,
                        TRAIT_DEFAULT => {
                            variant_traits::expand_default(context, &variant, attribute)?
                        }
                        TRAIT_TRY_FROM => {
                            variant_traits::expand_try_from(context, &variant, attribute)?
                        }
                        TRAIT_TRY_INTO => {
                            variant_traits::expand_try_into(context, &variant, attribute)?
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &attribute.ident,
                                "invalid trait name",
                            ));
                        }
                    };
                    implems.extend_traits(tokens);
                }
            };
        }
    }
    Ok(())
}
