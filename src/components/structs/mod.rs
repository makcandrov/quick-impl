use syn::DataStruct;

use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::methods::*;
use crate::idents::traits::*;
use crate::tokens::to_indexed_field_iter;

mod field_methods;
mod field_traits;
mod global_methods;

pub fn struct_impl(
    context: &Context<'_>,
    implems: &mut Implems,
    global_attributes: &Attributes,
    data_struct: &DataStruct,
) -> syn::Result<()> {
    for attribute in global_attributes.iter() {
        match &attribute.typ {
            AttributeType::Method(method_attr) => {
                let tokens = match attribute.ident.to_string().as_str() {
                    METHOD_NEW => global_methods::expand_new(
                        context,
                        attribute,
                        method_attr,
                        &data_struct.fields,
                    )?,
                    METHOD_INTO_PARTS => global_methods::expand_into_parts(
                        context,
                        attribute,
                        method_attr,
                        &data_struct.fields,
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
            AttributeType::Trait => todo!(),
        }
    }

    let indexed_fields = to_indexed_field_iter(&data_struct.fields).collect::<Vec<_>>();

    for indexed_field in &indexed_fields {
        let field_attributes = Attributes::from_attributes(&indexed_field.attrs)?;

        for attribute in field_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_FROM => field_methods::expand_from(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                            &indexed_fields,
                        )?,
                        METHOD_GET => field_methods::expand_get(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_CLONE => field_methods::expand_get_clone(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_MUT => field_methods::expand_get_mut(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_INTO => field_methods::expand_into(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_SET => field_methods::expand_set(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_TAKE => field_methods::expand_take(
                            context,
                            indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_WITH => field_methods::expand_with(
                            context,
                            indexed_field,
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
                        TRAIT_AS_MUT => {
                            field_traits::expand_as_mut(context, indexed_field, attribute)?
                        }
                        TRAIT_AS_REF => {
                            field_traits::expand_as_ref(context, indexed_field, attribute)?
                        }
                        TRAIT_BORROW => {
                            field_traits::expand_borrow(context, indexed_field, attribute)?
                        }
                        TRAIT_BORROW_MUT => {
                            field_traits::expand_borrow_mut(context, indexed_field, attribute)?
                        }
                        TRAIT_DEREF => {
                            field_traits::expand_deref(context, indexed_field, attribute)?
                        }
                        TRAIT_DEREF_MUT => {
                            field_traits::expand_deref_mut(context, indexed_field, attribute)?
                        }
                        TRAIT_FROM => field_traits::expand_from(
                            context,
                            indexed_field,
                            attribute,
                            &indexed_fields,
                        )?,
                        TRAIT_INTO => field_traits::expand_into(context, indexed_field, attribute)?,
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
