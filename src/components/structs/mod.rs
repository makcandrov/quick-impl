use syn::ItemStruct;

use crate::{
    expand::Implems,
    idents::{methods::*, traits::*},
    order::{AllOrders, Order},
    tokens::to_indexed_field_iter,
};

mod field_methods;
mod field_traits;
mod global_methods;
mod global_traits;

pub fn struct_impl(
    input: &ItemStruct,
    implems: &mut Implems,
    all_orders: &AllOrders,
) -> syn::Result<()> {
    for order in all_orders.global() {
        match order {
            Order::Method(order) => {
                let tokens = match order.ident.to_string().as_str() {
                    METHOD_FROM_TUPLE => global_methods::expand_from_tuple(input, order)?,
                    METHOD_INTO_PARTS => global_methods::expand_into_parts(input, order)?,
                    METHOD_NEW => global_methods::expand_new(input, order)?,
                    _ => {
                        return Err(syn::Error::new_spanned(&order.ident, "unknown method"));
                    }
                };
                implems.extend_methods(tokens);
            }
            Order::Trait(order) => {
                let tokens = match order.ident.to_string().as_str() {
                    TRAIT_FROM => global_traits::expand_from(input, order)?,
                    TRAIT_INTO => global_traits::expand_into(input, order)?,
                    _ => {
                        return Err(syn::Error::new_spanned(&order.ident, "unknown trait"));
                    }
                };
                implems.extend_traits(tokens);
            }
        }
    }

    for indexed_field in to_indexed_field_iter(&input.fields) {
        for order in all_orders.per_item(indexed_field.index) {
            match order {
                Order::Method(order) => {
                    let tokens = match order.ident.to_string().as_str() {
                        METHOD_FROM => field_methods::expand_from(input, &indexed_field, order)?,
                        METHOD_GET => field_methods::expand_get(input, &indexed_field, order)?,
                        METHOD_GET_CLONE => {
                            field_methods::expand_get_clone(input, &indexed_field, order)?
                        }
                        METHOD_GET_MUT => {
                            field_methods::expand_get_mut(input, &indexed_field, order)?
                        }
                        METHOD_INTO => field_methods::expand_into(input, &indexed_field, order)?,
                        METHOD_REPLACE => {
                            field_methods::expand_replace(input, &indexed_field, order)?
                        }
                        METHOD_SET => field_methods::expand_set(input, &indexed_field, order)?,
                        METHOD_TAKE => field_methods::expand_take(input, &indexed_field, order)?,
                        METHOD_WITH => field_methods::expand_with(input, &indexed_field, order)?,
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &order.ident,
                                "invalid method name",
                            ));
                        }
                    };
                    implems.extend_methods(tokens);
                }
                Order::Trait(order) => {
                    let tokens = match order.ident.to_string().as_str() {
                        TRAIT_AS_MUT => field_traits::expand_as_mut(input, &indexed_field, order)?,
                        TRAIT_AS_REF => field_traits::expand_as_ref(input, &indexed_field, order)?,
                        TRAIT_BORROW => field_traits::expand_borrow(input, &indexed_field, order)?,
                        TRAIT_BORROW_MUT => {
                            field_traits::expand_borrow_mut(input, &indexed_field, order)?
                        }
                        TRAIT_DEREF => field_traits::expand_deref(input, &indexed_field, order)?,
                        TRAIT_DEREF_MUT => {
                            field_traits::expand_deref_mut(input, &indexed_field, order)?
                        }
                        TRAIT_FROM => field_traits::expand_from(input, &indexed_field, order)?,
                        TRAIT_INTO => field_traits::expand_into(input, &indexed_field, order)?,
                        _ => {
                            return Err(syn::Error::new_spanned(&order.ident, "invalid trait name"));
                        }
                    };
                    implems.extend_traits(tokens);
                }
            };
        }
    }
    Ok(())
}
