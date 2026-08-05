//! Raw identifiers are valid field and variant names, but `r#` cannot appear in the middle of an
//! identifier, so it must be stripped when building generated names.

#![allow(non_camel_case_types)]

use core::ops::{Deref, DerefMut};

use quick_impl::{quick_impl, quick_impl_all};

#[test]
fn test_raw_named_fields() {
    #[derive(Debug, Eq, PartialEq)]
    #[quick_impl(pub new, pub from_tuple, pub into_parts)]
    struct Test {
        #[quick_impl(
            pub get,
            pub get_clone = "clone_{}",
            pub get_mut,
            pub into,
            pub replace,
            pub set,
            pub take,
            pub with,
            impl Deref,
            impl DerefMut,
            impl AsRef,
            impl AsMut,
            impl Borrow,
            impl BorrowMut,
        )]
        r#type: usize,
        r#struct: usize,
    }

    let mut test = Test::new(1, 2);

    assert_eq!(*test.get_type(), 1);
    assert_eq!(test.clone_type(), 1);
    assert_eq!(*test.get_type_mut(), 1);
    assert_eq!(test.replace_type(3), 1);
    assert_eq!(test.take_type(), 3);
    assert_eq!(test.set_type(4), &mut Test::new(4, 2));
    assert_eq!(Test::new(1, 2).with_type(5), Test::new(5, 2));
    assert_eq!(Test::new(1, 2).into_type(), 1);

    assert_eq!(*Deref::deref(&test), 4);
    assert_eq!(*DerefMut::deref_mut(&mut test), 4);
    assert_eq!(AsRef::<usize>::as_ref(&test), &4);
    assert_eq!(AsMut::<usize>::as_mut(&mut test), &mut 4);
    assert_eq!(<Test as core::borrow::Borrow<usize>>::borrow(&test), &4);
    assert_eq!(
        <Test as core::borrow::BorrowMut<usize>>::borrow_mut(&mut test),
        &mut 4,
    );

    assert_eq!(Test::from_tuple((1, 2)), Test::new(1, 2));
    assert_eq!(Test::new(1, 2).into_parts(), (1, 2));
}

#[test]
fn test_raw_variants() {
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
            impl From,
            impl TryFrom,
        )]
        r#fn(usize),

        #[quick_impl(pub is, impl Default)]
        r#struct,
    }

    let mut test = Test::r#fn(12);

    assert!(test.is_fn());
    assert!(!test.is_struct());
    assert!(test.is_fn_and(|x| *x == 12));
    assert_eq!(test.as_fn(), Some(&12));
    assert_eq!(test.as_fn_mut(), Some(&mut 12));
    assert_eq!(test.set_fn(13), Test::r#fn(12));

    assert_eq!(Test::from_fn(12), Test::r#fn(12));
    assert_eq!(Test::from(12usize), Test::r#fn(12));
    assert_eq!(Test::default(), Test::r#struct);

    assert_eq!(Test::r#fn(12).into_fn(), Some(12));
    assert_eq!(Test::r#fn(12).try_into_fn(), Ok(12));
    assert_eq!(usize::try_from(Test::r#fn(12)), Ok(12));
    assert_eq!(Test::r#struct.try_into_fn(), Err(Test::r#struct));

    let inspected = Test::r#fn(12).inspect_fn(|x| assert_eq!(*x, 12));
    assert_eq!(inspected, Test::r#fn(12));
}

#[test]
fn test_raw_variant_camel_case() {
    #[derive(Debug, Eq, PartialEq)]
    #[quick_impl_all(pub is)]
    enum Test {
        r#Struct,
        r#RawVariant,
    }

    assert!(Test::r#Struct.is_struct());
    assert!(Test::r#RawVariant.is_raw_variant());
}

#[test]
fn test_raw_name_placeholder() {
    #[quick_impl]
    struct Test {
        // `{}` expands to the unraw name, so it must be escaped again to stay a valid identifier.
        #[quick_impl(pub get = "r#{}", pub get_mut = "{}_mut")]
        r#type: usize,
    }

    let mut test = Test { r#type: 12 };
    assert_eq!(*test.r#type(), 12);
    assert_eq!(*test.type_mut(), 12);
}

#[test]
fn test_raw_struct_and_field() {
    #[derive(Debug, Eq, PartialEq)]
    #[quick_impl(pub new)]
    struct r#struct {
        #[quick_impl(pub get, impl From, impl Into)]
        r#match: usize,
    }

    let test = r#struct::new(12);
    assert_eq!(*test.get_match(), 12);
    assert_eq!(r#struct::from(12), r#struct::new(12));
    assert_eq!(usize::from(r#struct::new(12)), 12);
}
