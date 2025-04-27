use syn::{Attribute, ItemEnum, ItemStruct};

use crate::{
    idents::{HELPER_QUICK_IMPL, HELPER_QUICK_IMPL_ALL},
    input::Input,
};

#[derive(Clone, Default)]
pub struct Attrs {
    pub quick_impl: Vec<Attribute>,
    pub quick_impl_all: Vec<Attribute>,
}

impl Attrs {
    pub fn new(attrs_raw: &mut Vec<Attribute>) -> Self {
        let mut attrs = Self::default();
        attrs.extract(attrs_raw);
        attrs
    }

    pub fn extract(&mut self, attrs_raw: &mut Vec<Attribute>) {
        let mut i = 0;
        while i < attrs_raw.len() {
            let attr = &attrs_raw[i];
            let path = attr.path();
            if path.is_ident(HELPER_QUICK_IMPL) {
                self.quick_impl.push(attrs_raw.remove(i));
            } else if path.is_ident(HELPER_QUICK_IMPL_ALL) {
                self.quick_impl_all.push(attrs_raw.remove(i));
            } else {
                i += 1;
            }
        }
    }
}

#[derive(Clone)]
pub struct AllAttrs {
    /// Orders directly above the enum/struct.
    ///
    /// Does _not_ include the main macro invocation attribute.
    pub global: Attrs,

    /// Per variant/field attribues.
    ///
    /// The vector size _must_ match the variants/fields number.
    pub per_item: Vec<Attrs>,
}

impl AllAttrs {
    fn new<'a>(
        global_raw: &'a mut Vec<Attribute>,
        per_item_raw: impl IntoIterator<Item = &'a mut Vec<Attribute>>,
    ) -> Self {
        let mut global = Attrs::default();
        global.extract(global_raw);
        Self { global, per_item: per_item_raw.into_iter().map(Attrs::new).collect() }
    }

    pub fn extract_from_input(input: &mut Input) -> Self {
        match input {
            Input::Enum(item) => Self::extract_from_item_enum(item),
            Input::Struct(item) => Self::extract_from_item_struct(item),
        }
    }

    pub fn extract_from_item_enum(item_enum: &mut ItemEnum) -> Self {
        Self::new(&mut item_enum.attrs, item_enum.variants.iter_mut().map(|field| &mut field.attrs))
    }

    pub fn extract_from_item_struct(item_struct: &mut ItemStruct) -> Self {
        Self::new(
            &mut item_struct.attrs,
            item_struct.fields.iter_mut().map(|field| &mut field.attrs),
        )
    }
}
