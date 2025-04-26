use syn::ItemEnum;

use crate::{
    attr::{AttrKind, Attrs},
    expand::Implems,
    idents::{methods::*, traits::*},
};

mod variant_methods;
mod variant_traits;

pub fn enum_impl(
    input: &mut ItemEnum,
    implems: &mut Implems,
    all_attrs: &Attrs,
    glob_attrs: &Attrs,
) -> syn::Result<()> {
    #[expect(clippy::never_loop)]
    for attribute in glob_attrs.iter() {
        match &attribute.kind {
            AttrKind::Method(_) => {
                return Err(syn::Error::new_spanned(&attribute.ident, "invalid method name"));
            }
            AttrKind::Trait => {
                return Err(syn::Error::new_spanned(&attribute.ident, "invalid trait name"));
            }
        }
    }

    let all_variants_attrs: Vec<_> = input
        .variants
        .iter_mut()
        .map(|variant| Attrs::take_from(&mut variant.attrs, false))
        .collect::<Result<_, _>>()?;

    for (variant, variant_attrs) in input.variants.iter().zip(all_variants_attrs) {
        for attribute in all_attrs.iter().chain(variant_attrs.iter()) {
            match &attribute.kind {
                AttrKind::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => variant_methods::expand_as_ref_mut(
                            input,
                            variant,
                            attribute,
                            method_attr,
                        )?,
                        METHOD_AS_REF => {
                            variant_methods::expand_as_ref(input, variant, attribute, method_attr)?
                        }
                        METHOD_FROM => {
                            variant_methods::expand_from(input, variant, attribute, method_attr)?
                        }
                        METHOD_INSPECT => {
                            variant_methods::expand_inspect(input, variant, attribute, method_attr)?
                        }
                        METHOD_INTO => {
                            variant_methods::expand_into(input, variant, attribute, method_attr)?
                        }
                        METHOD_IS_AND => {
                            variant_methods::expand_is_and(input, variant, attribute, method_attr)?
                        }
                        METHOD_IS => {
                            variant_methods::expand_is(input, variant, attribute, method_attr)?
                        }
                        METHOD_SET => {
                            variant_methods::expand_set(input, variant, attribute, method_attr)?
                        }
                        METHOD_TRY_INTO => variant_methods::expand_try_into(
                            input,
                            variant,
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
                        TRAIT_DEFAULT => variant_traits::expand_default(input, variant, attribute)?,
                        TRAIT_FROM => variant_traits::expand_from(input, variant, attribute)?,
                        TRAIT_TRY_FROM => {
                            variant_traits::expand_try_from(input, variant, attribute)?
                        }
                        TRAIT_TRY_INTO => {
                            variant_traits::expand_try_into(input, variant, attribute)?
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
