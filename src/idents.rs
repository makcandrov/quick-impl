pub const HELPER_QUICK_IMPL: &str = "quick_impl";
pub const HELPER_QUICK_IMPL_ALL: &str = "quick_impl_all";

pub const ARGUMENT: &str = "arg";

pub mod config {
    pub const CONFIG_DOC: &str = "doc";
    pub const CONFIG_NAME: &str = "name";
}

pub mod methods {
    pub const METHOD_AS_REF_MUT: &str = "as_ref_mut";
    pub const METHOD_AS_REF: &str = "as_ref";
    pub const METHOD_FROM: &str = "from";
    pub const METHOD_FROM_TUPLE: &str = "from_tuple";
    pub const METHOD_GET: &str = "get";
    pub const METHOD_GET_CLONE: &str = "get_clone";
    pub const METHOD_GET_MUT: &str = "get_mut";
    pub const METHOD_INSPECT: &str = "inspect";
    pub const METHOD_INTO: &str = "into";
    pub const METHOD_INTO_PARTS: &str = "into_parts";
    pub const METHOD_IS_AND: &str = "is_and";
    pub const METHOD_IS: &str = "is";
    pub const METHOD_NEW: &str = "new";
    pub const METHOD_REPLACE: &str = "replace";
    pub const METHOD_SET: &str = "set";
    pub const METHOD_TAKE: &str = "take";
    pub const METHOD_TRY_INTO: &str = "try_into";
    pub const METHOD_WITH: &str = "with";
}

pub mod traits {
    pub const TRAIT_AS_MUT: &str = "AsMut";
    pub const TRAIT_AS_REF: &str = "AsRef";
    pub const TRAIT_BORROW: &str = "Borrow";
    pub const TRAIT_BORROW_MUT: &str = "BorrowMut";
    pub const TRAIT_DEFAULT: &str = "Default";
    pub const TRAIT_DEREF: &str = "Deref";
    pub const TRAIT_DEREF_MUT: &str = "DerefMut";
    pub const TRAIT_FROM: &str = "From";
    pub const TRAIT_INTO: &str = "Into";
    pub const TRAIT_TRY_FROM: &str = "TryFrom";
    pub const TRAIT_TRY_INTO: &str = "TryInto";
}

pub fn ident_list_message<'a>(idents: impl IntoIterator<Item = &'a str>) -> String {
    idents.into_iter().map(|ident| format!("`{ident}`")).collect::<Vec<_>>().join(", ")
}
