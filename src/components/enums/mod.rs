use syn::ItemEnum;

use crate::{
    expand::Implems,
    idents::{ident_list_message, methods::*, traits::*},
    order::{AllOrders, Order},
};

mod variant_methods;
mod variant_traits;

pub fn enum_impl(
    input: &ItemEnum,
    implems: &mut Implems,
    all_orders: &AllOrders,
) -> syn::Result<()> {
    #[expect(clippy::never_loop)]
    for order in all_orders.global() {
        match order {
            Order::Method(order) => {
                return Err(syn::Error::new_spanned(
                    &order.ident,
                    "unknown method (there are currently no global method available on enums)",
                ));
            }
            Order::Trait(order) => {
                return Err(syn::Error::new_spanned(
                    &order.ident,
                    "unknown trait (there are currently no global trait available on enums)",
                ));
            }
        }
    }

    for (variant_index, variant) in input.variants.iter().enumerate() {
        for order in all_orders.per_item(variant_index) {
            match order {
                Order::Method(order) => {
                    let tokens = match order.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => {
                            variant_methods::expand_as_ref_mut(input, variant, order)?
                        }
                        METHOD_AS_REF => variant_methods::expand_as_ref(input, variant, order)?,
                        METHOD_FROM => variant_methods::expand_from(input, variant, order)?,
                        METHOD_INSPECT => variant_methods::expand_inspect(input, variant, order)?,
                        METHOD_INTO => variant_methods::expand_into(input, variant, order)?,
                        METHOD_IS_AND => variant_methods::expand_is_and(input, variant, order)?,
                        METHOD_IS => variant_methods::expand_is(input, variant, order)?,
                        METHOD_SET => variant_methods::expand_set(input, variant, order)?,
                        METHOD_TRY_INTO => variant_methods::expand_try_into(input, variant, order)?,
                        _ => {
                            let all_methods = ident_list_message([
                                METHOD_AS_REF_MUT,
                                METHOD_AS_REF,
                                METHOD_FROM,
                                METHOD_INSPECT,
                                METHOD_INTO,
                                METHOD_IS_AND,
                                METHOD_IS,
                                METHOD_SET,
                                METHOD_TRY_INTO,
                            ]);
                            return Err(syn::Error::new_spanned(
                                &order.ident,
                                format!(
                                    "unknown method `{}`, expected one of {}",
                                    order.ident, all_methods
                                ),
                            ));
                        }
                    };
                    implems.extend_methods(tokens);
                }
                Order::Trait(order) => {
                    let tokens = match order.ident.to_string().as_str() {
                        TRAIT_DEFAULT => variant_traits::expand_default(input, variant, order)?,
                        TRAIT_FROM => variant_traits::expand_from(input, variant, order)?,
                        TRAIT_TRY_FROM => variant_traits::expand_try_from(input, variant, order)?,
                        TRAIT_TRY_INTO => variant_traits::expand_try_into(input, variant, order)?,
                        _ => {
                            let all_traits = ident_list_message([
                                TRAIT_DEFAULT,
                                TRAIT_FROM,
                                TRAIT_TRY_FROM,
                                TRAIT_TRY_INTO,
                            ]);
                            return Err(syn::Error::new_spanned(
                                &order.ident,
                                format!(
                                    "unknown trait `{}`, expected one of {}",
                                    order.ident, all_traits
                                ),
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
