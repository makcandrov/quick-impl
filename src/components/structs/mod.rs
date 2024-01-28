use syn::DataStruct;

use self::traits::struct_trait_deref;
use crate::attributes::{AttributeType, Attributes};
use crate::expand::{Context, Implems};
use crate::idents::TRAIT_DEREF;

mod traits;

pub fn struct_impl(context: &Context<'_>, implems: &mut Implems, data_struct: &DataStruct) -> syn::Result<()> {
    for (field_index, field) in data_struct.fields.iter().enumerate() {
        let field_attributes = Attributes::from_attributes(&field.attrs)?;

        for attribute in field_attributes.iter() {
            match &attribute.typ {
                AttributeType::Method(method_attr) => {
                    return Err(syn::Error::new_spanned(&attribute.ident, "Invalid method name."));
                    // let tokens = match attribute.ident.to_string().as_str() {
                    //     _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid method name.")),
                    // };
                    // implems.extend_methods(tokens);
                },
                AttributeType::Trait => {
                    let tokens = match attribute.ident.to_string().as_str() {
                        TRAIT_DEREF => struct_trait_deref(context, field, field_index, attribute)?,
                        _ => return Err(syn::Error::new_spanned(&attribute.ident, "Invalid trait name.")),
                    };
                    implems.extend_traits(tokens);
                },
            };
        }
    }
    Ok(())
}
