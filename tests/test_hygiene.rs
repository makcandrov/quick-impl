//! The generated code must not depend on anything the caller happens to have in scope.

use quick_impl::{quick_impl, quick_impl_all};

#[allow(dead_code)]
pub struct Option;

#[allow(dead_code)]
pub struct Some;

#[allow(dead_code)]
pub struct None;

#[allow(dead_code)]
pub struct Ok;

#[allow(dead_code)]
pub struct Err;

#[allow(dead_code)]
pub type Result<T> = ::core::result::Result<T, ()>;

#[allow(dead_code)]
pub trait Default {}

#[allow(dead_code)]
pub trait Clone {}

mod core {}

mod std {}

#[test]
fn test_shadowed_prelude_enum() {
    #[derive(Debug, Eq, PartialEq)]
    #[quick_impl]
    enum Test {
        #[quick_impl(
            pub is,
            pub is_and,
            pub as_ref,
            pub as_ref_mut,
            pub from,
            pub into,
            pub set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom,
        )]
        A(usize),
        #[allow(dead_code)]
        B,
    }

    let mut a = Test::A(12);
    assert!(a.is_a());
    assert!(a.is_a_and(|x| *x == 12));
    assert_eq!(a.as_a(), ::core::option::Option::Some(&12));
    assert_eq!(a.as_a_mut(), ::core::option::Option::Some(&mut 12));
    assert_eq!(Test::B.as_a(), ::core::option::Option::None);
    assert_eq!(a.set_a(13), Test::A(12));

    assert_eq!(Test::from_a(12), Test::A(12));
    assert_eq!(
        <Test as ::core::convert::From<usize>>::from(12),
        Test::A(12)
    );
    assert_eq!(<Test as ::core::default::Default>::default(), Test::A(0));

    assert_eq!(Test::A(12).into_a(), ::core::option::Option::Some(12));
    assert_eq!(
        Test::A(12).try_into_a(),
        ::core::result::Result::Ok::<usize, Test>(12),
    );
    assert_eq!(
        <usize as ::core::convert::TryFrom<Test>>::try_from(Test::A(12)),
        ::core::result::Result::Ok::<usize, Test>(12),
    );
    assert_eq!(
        Test::B.try_into_a(),
        ::core::result::Result::Err::<usize, Test>(Test::B),
    );
    assert_eq!(Test::A(12).inspect_a(|_| {}), Test::A(12));
}

#[test]
fn test_shadowed_prelude_struct() {
    #[derive(Debug, Eq, PartialEq)]
    #[quick_impl_all(pub get, pub get_clone = "clone_{}", pub get_mut, pub set, pub take, pub with)]
    #[quick_impl(pub new, pub from_tuple, pub into_parts, impl From, impl Into)]
    struct Test {
        a: usize,
        b: usize,
    }

    let mut test = Test::new(1, 2);
    assert_eq!(*test.get_a(), 1);
    assert_eq!(test.clone_b(), 2);
    assert_eq!(*test.get_a_mut(), 1);
    assert_eq!(test.take_a(), 1);
    assert_eq!(test.set_a(1), &mut Test::new(1, 2));
    assert_eq!(Test::new(1, 2).with_b(3), Test::new(1, 3));
    assert_eq!(Test::from_tuple((1, 2)), Test::new(1, 2));
    assert_eq!(Test::new(1, 2).into_parts(), (1, 2));
}
