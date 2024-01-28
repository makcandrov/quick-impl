use syn::DataStruct;

use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::fields::to_indexed_field_iter;
use crate::idents::methods::METHOD_GET;
use crate::idents::traits::{TRAIT_DEREF, TRAIT_DEREF_MUT};

mod methods;
mod traits;
use traits::{struct_trait_deref, struct_trait_deref_mut};

use self::methods::struct_method_get;

pub fn struct_impl(context: &Context<'_>, implems: &mut Implems, data_struct: &DataStruct) -> syn::Result<()> {
    for indexed_field in to_indexed_field_iter(&data_struct.fields) {
        let field_attributes = Attributes::from_attributes(&indexed_field.attrs)?;

        for attribute in field_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        METHOD_GET => struct_method_get(context, &indexed_field, attribute, method_attr)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid method name.")),
                    };
                    implems.extend_methods(tokens);
                },
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_DEREF => struct_trait_deref(context, &indexed_field, attribute)?,
                        TRAIT_DEREF_MUT => struct_trait_deref_mut(context, &indexed_field, attribute)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid trait name.")),
                    };
                    implems.extend_traits(tokens);
                },
            };
        }
    }
    Ok(())
}
