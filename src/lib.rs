#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod attr;
mod components;
mod config;
mod ctx;
mod expand;
mod idents;
mod input;
mod order;
mod respan;
mod tokens;
mod utils;

/// Quickly generate common methods and trait implementations on enums or structs.
///
/// ## Enum example
/// ```rust
/// use quick_impl::quick_impl;
///
/// #[quick_impl]
/// enum MyEnum {
///     #[quick_impl(pub const is, impl Default)]
///     Variant1,
///     #[quick_impl(pub as_ref, pub(crate) as_ref_mut, impl From)]
///     Variant2(i32),
/// }
///
/// let a = MyEnum::default();
/// assert!(a.is_variant1());
///
/// let mut b = MyEnum::from(42);
/// assert_eq!(*b.as_variant2().unwrap(), 42);
/// ```
///
/// ## Struct example
/// ```rust
/// use quick_impl::quick_impl;
///
/// #[quick_impl(pub const new)]
/// struct MyStruct {
///     #[quick_impl(pub get)]
///     field1: i32,
///
///     #[quick_impl(pub get_mut, impl Into)]
///     field2: String,
/// }
///
/// let mut instance = MyStruct::new(1, "hello".to_string());
/// assert_eq!(*instance.get_field1(), 1);
/// instance.get_field2_mut().push_str(" world");
/// assert_eq!(&String::from(instance), "hello world");
/// ```
#[proc_macro_attribute]
pub fn quick_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::expand_qi(args.into(), input.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// Quickly generate common methods and trait implementations on enums or structs on all the
/// variants or fields.
///
/// [`quick_impl_all`](macro@quick_impl_all) can be combined with [`quick_impl`](macro@quick_impl)
/// in any order.
///
/// ## Enum example
/// ```rust
/// use quick_impl::quick_impl_all;
///
/// #[quick_impl_all(pub const is)]
/// enum MyEnum {
///     Variant1,
///     #[quick_impl(pub as_ref, impl From)]
///     Variant2(i32),
/// }
///
/// let a = MyEnum::Variant1;
/// assert!(a.is_variant1());
///
/// let mut b = MyEnum::from(42);
/// assert!(b.is_variant2());
/// ```
///
/// ## Struct example
/// ```rust
/// use quick_impl::quick_impl;
///
/// #[quick_impl(pub const new)]
/// #[quick_impl_all(pub get = "{}", pub get_mut = "{}_mut")]
/// struct MyStruct {
///     field1: i32,
///
///     #[quick_impl(impl Into)]
///     field2: String,
/// }
///
/// let mut instance = MyStruct::new(1, "hello".to_string());
/// assert_eq!(*instance.field1(), 1);
/// instance.field2_mut().push_str(" world");
/// assert_eq!(&String::from(instance), "hello world");
/// ```
#[proc_macro_attribute]
pub fn quick_impl_all(args: TokenStream, input: TokenStream) -> TokenStream {
    expand::expand_qia(args.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
