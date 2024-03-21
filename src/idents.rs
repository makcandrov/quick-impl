pub const MACRO_DERIVE_HELPER: &str = "quick_impl";
pub const ARGUMENT: &str = "arg";

pub mod config {
    pub const CONFIG_DOC: &str = "doc";
    pub const CONFIG_NAME: &str = "name";
}

pub mod methods {
    pub const METHOD_AS_REF: &str = "as_ref";
    pub const METHOD_AS_REF_MUT: &str = "as_ref_mut";
    pub const METHOD_FROM: &str = "from";
    pub const METHOD_GET: &str = "get";
    pub const METHOD_GET_MUT: &str = "get_mut";
    pub const METHOD_INTO: &str = "into";
    pub const METHOD_IS: &str = "is";
    pub const METHOD_SET: &str = "set";
    pub const METHOD_WITH: &str = "with";
}

pub mod traits {
    pub const TRAIT_AS_MUT: &str = "AsMut";
    pub const TRAIT_AS_REF: &str = "AsRef";
    pub const TRAIT_DEREF: &str = "Deref";
    pub const TRAIT_DEREF_MUT: &str = "DerefMut";
    pub const TRAIT_FROM: &str = "From";
    pub const TRAIT_INTO: &str = "Into";
}
