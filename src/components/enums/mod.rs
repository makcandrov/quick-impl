use syn::DataEnum;

use self::methods::{enum_method_as_ref, enum_method_as_ref_mut, enum_method_from, enum_method_into, enum_method_is};
use self::traits::enum_trait_from;
use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::{METHOD_AS_REF, METHOD_AS_REF_MUT, METHOD_FROM, METHOD_INTO, METHOD_IS};

mod methods;
mod traits;

pub fn enum_impl(context: &Context<'_>, implems: &mut Implems, data_enum: &DataEnum) -> syn::Result<()> {
    for variant in &data_enum.variants {
        let variant_attributes = Attributes::from_attributes(&variant.attrs)?;
        let fields = &variant.fields;

        for attribute in variant_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method { visibility, constant } => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_AS_REF_MUT => {
                            enum_method_as_ref_mut(context, &variant.ident, fields, attribute, visibility, *constant)?
                        },
                        METHOD_AS_REF => {
                            enum_method_as_ref(context, &variant.ident, fields, attribute, visibility, *constant)?
                        },
                        METHOD_FROM => {
                            enum_method_from(context, &variant.ident, fields, attribute, visibility, *constant)?
                        },
                        METHOD_INTO => {
                            enum_method_into(context, &variant.ident, fields, attribute, visibility, *constant)?
                        },
                        METHOD_IS => enum_method_is(context, &variant.ident, fields, attribute, visibility, *constant)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid method name.")),
                    };
                    implems.extend_methods(tokens);
                },
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_FROM => enum_trait_from(context, &variant.ident, fields, attribute)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid trait name.")),
                    };
                    implems.extend_traits(tokens);
                },
            };
        }
    }
    Ok(())
}
