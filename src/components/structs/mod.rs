use syn::ItemStruct;

use crate::{
    attr::{AttrKind, Attrs},
    expand::Implems,
    idents::{methods::*, traits::*},
    tokens::to_indexed_field_iter,
};

mod field_methods;
mod field_traits;
mod global_methods;
mod global_traits;

pub fn struct_impl(
    input: &mut ItemStruct,
    implems: &mut Implems,
    global_attributes: &Attrs,
) -> syn::Result<()> {
    for attribute in global_attributes.iter() {
        match &attribute.kind {
            AttrKind::Method(method_attr) => {
                let tokens = match attribute.ident.to_string().as_str() {
                    METHOD_FROM_TUPLE => {
                        global_methods::expand_from_tuple(input, attribute, method_attr)?
                    }
                    METHOD_INTO_PARTS => {
                        global_methods::expand_into_parts(input, attribute, method_attr)?
                    }
                    METHOD_NEW => global_methods::expand_new(input, attribute, method_attr)?,
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &attribute.ident,
                            "invalid method name",
                        ));
                    }
                };
                implems.extend_methods(tokens);
            }
            AttrKind::Trait => {
                let tokens = match attribute.ident.to_string().as_str() {
                    TRAIT_FROM => global_traits::expand_from(input, attribute, &input.fields)?,
                    TRAIT_INTO => global_traits::expand_into(input, attribute, &input.fields)?,
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &attribute.ident,
                            "invalid trait name",
                        ));
                    }
                };
                implems.extend_traits(tokens);
            }
        }
    }

    let all_fields_attrs: Vec<_> = input
        .fields
        .iter_mut()
        .map(|field| Attrs::take_from(&mut field.attrs))
        .collect::<Result<_, _>>()?;

    for (indexed_field, field_attrs) in to_indexed_field_iter(&input.fields).zip(all_fields_attrs) {
        for attribute in field_attrs.iter() {
            match &attribute.kind {
                AttrKind::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_FROM => field_methods::expand_from(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET => field_methods::expand_get(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_CLONE => field_methods::expand_get_clone(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_GET_MUT => field_methods::expand_get_mut(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_INTO => field_methods::expand_into(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_REPLACE => field_methods::expand_replace(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_SET => field_methods::expand_set(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_TAKE => field_methods::expand_take(
                            input,
                            &indexed_field,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_WITH => field_methods::expand_with(
                            input,
                            &indexed_field,
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
                AttrKind::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_AS_MUT => {
                            field_traits::expand_as_mut(input, &indexed_field, attribute)?
                        }
                        TRAIT_AS_REF => {
                            field_traits::expand_as_ref(input, &indexed_field, attribute)?
                        }
                        TRAIT_BORROW => {
                            field_traits::expand_borrow(input, &indexed_field, attribute)?
                        }
                        TRAIT_BORROW_MUT => {
                            field_traits::expand_borrow_mut(input, &indexed_field, attribute)?
                        }
                        TRAIT_DEREF => {
                            field_traits::expand_deref(input, &indexed_field, attribute)?
                        }
                        TRAIT_DEREF_MUT => {
                            field_traits::expand_deref_mut(input, &indexed_field, attribute)?
                        }
                        TRAIT_FROM => field_traits::expand_from(input, &indexed_field, attribute)?,
                        TRAIT_INTO => field_traits::expand_into(input, &indexed_field, attribute)?,
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
