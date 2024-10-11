use syn::DataEnum;

use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::{methods::*, traits::*};

mod methods;
use methods::*;

mod traits;
use traits::*;

pub fn enum_impl(
    context: &Context<'_>,
    implems: &mut Implems,
    data_enum: &DataEnum,
) -> syn::Result<()> {
    for variant in &data_enum.variants {
        let variant_attributes = Attributes::from_attributes(&variant.attrs)?;

        for attribute in variant_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => {
                            enum_method_as_ref_mut(context, &variant, attribute, method_attr)?
                        }
                        METHOD_AS_REF => {
                            enum_method_as_ref(context, &variant, attribute, method_attr)?
                        }
                        METHOD_FROM => enum_method_from(context, &variant, attribute, method_attr)?,
                        METHOD_INTO => enum_method_into(context, &variant, attribute, method_attr)?,
                        METHOD_IS => enum_method_is(context, &variant, attribute, method_attr)?,
                        METHOD_SET => enum_method_set(context, &variant, attribute, method_attr)?,
                        METHOD_TRY_INTO => {
                            enum_method_try_into(context, &variant, attribute, method_attr)?
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &attribute.ident,
                                "Invalid method name.",
                            ))
                        }
                    };
                    implems.extend_methods(tokens);
                }
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_FROM => enum_trait_from(context, &variant, attribute)?,
                        TRAIT_DEFAULT => enum_trait_default(context, &variant, attribute)?,
                        TRAIT_TRY_INTO => enum_trait_try_into(context, &variant, attribute)?,
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &attribute.ident,
                                "Invalid trait name.",
                            ))
                        }
                    };
                    implems.extend_traits(tokens);
                }
            };
        }
    }
    Ok(())
}
