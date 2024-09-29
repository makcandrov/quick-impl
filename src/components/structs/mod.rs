use syn::DataStruct;

use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::methods::{
    METHOD_GET, METHOD_GET_CLONE, METHOD_GET_MUT, METHOD_INTO, METHOD_SET, METHOD_WITH,
};
use crate::idents::traits::{TRAIT_AS_MUT, TRAIT_AS_REF, TRAIT_DEREF, TRAIT_DEREF_MUT, TRAIT_INTO};
use crate::tokens::to_indexed_field_iter;

mod methods;
mod traits;

pub fn struct_impl(
    context: &Context<'_>,
    implems: &mut Implems,
    data_struct: &DataStruct,
) -> syn::Result<()> {
    for indexed_field in to_indexed_field_iter(&data_struct.fields) {
        let field_attributes = Attributes::from_attributes(&indexed_field.attrs)?;

        for attribute in field_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_GET => methods::struct_method_get(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_CLONE => methods::struct_method_get_clone(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_MUT => methods::struct_method_get_mut(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_INTO => methods::struct_method_into(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_SET => methods::struct_method_set(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_WITH => methods::struct_method_with(
                            context,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
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
                        TRAIT_AS_MUT => {
                            traits::struct_trait_as_mut(context, &indexed_field, attribute)?
                        }
                        TRAIT_AS_REF => {
                            traits::struct_trait_as_ref(context, &indexed_field, attribute)?
                        }
                        TRAIT_DEREF => {
                            traits::struct_trait_deref(context, &indexed_field, attribute)?
                        }
                        TRAIT_DEREF_MUT => {
                            traits::struct_trait_deref_mut(context, &indexed_field, attribute)?
                        }
                        TRAIT_INTO => {
                            traits::struct_trait_into(context, &indexed_field, attribute)?
                        }
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
