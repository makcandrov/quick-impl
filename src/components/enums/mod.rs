use syn::DataEnum;

use self::methods::{enum_method_as_ref, enum_method_as_ref_mut, enum_method_from, enum_method_into, enum_method_is};
use self::traits::{enum_trait_default, enum_trait_from};
use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::methods::{METHOD_AS_REF, METHOD_AS_REF_MUT, METHOD_FROM, METHOD_INTO, METHOD_IS};
use crate::idents::traits::{TRAIT_DEFAULT, TRAIT_FROM};

mod methods;
mod traits;

pub fn enum_impl(context: &Context<'_>, implems: &mut Implems, data_enum: &DataEnum) -> syn::Result<()> {
    for variant in &data_enum.variants {
        let variant_attributes = Attributes::from_attributes(&variant.attrs)?;

        for attribute in variant_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => enum_method_as_ref_mut(context, &variant, attribute, method_attr)?,
                        METHOD_AS_REF => enum_method_as_ref(context, &variant, attribute, method_attr)?,
                        METHOD_FROM => enum_method_from(context, &variant, attribute, method_attr)?,
                        METHOD_INTO => enum_method_into(context, &variant, attribute, method_attr)?,
                        METHOD_IS => enum_method_is(context, &variant, attribute, method_attr)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid method name.")),
                    };
                    implems.extend_methods(tokens);
                },
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_FROM => enum_trait_from(context, &variant, attribute)?,
                        TRAIT_DEFAULT => enum_trait_default(context, &variant, attribute)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid trait name.")),
                    };
                    implems.extend_traits(tokens);
                },
            };
        }
    }
    Ok(())
}
