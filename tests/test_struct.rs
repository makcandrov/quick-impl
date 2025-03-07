use core::ops::{Deref, DerefMut};

use quick_impl::QuickImpl;

#[test]
fn test_struct_single_named() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub into_parts)]
    struct Test {
        #[quick_impl(
            pub const get = "{}",
            pub get_clone,
            get_mut,
            const into,
            pub(crate) set,
            pub take,
            pub(crate) const with,
            pub from,
            impl Deref,
            impl DerefMut,
            impl Into,
            impl From,
            impl AsRef,
            impl AsMut,
            impl Borrow,
            impl BorrowMut,
        )]
        a: usize,
        #[allow(dead_code)]
        b: usize,
    }

    let a = Test { a: 12, b: 14 };
    assert_eq!(*a.a(), 12);
    assert_eq!(*<Test as Deref>::deref(&a), 12);

    let a = a.with_a(13);
    assert_eq!(a.into_a(), 13);

    let mut a = Test { a: 12, b: 14 };
    assert_eq!(*<Test as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(a.set_a(13), 12);
    assert_eq!(*a.get_a_mut(), 13);

    *a.get_a_mut() = 14;
    assert_eq!(Into::<usize>::into(a.clone()), 14);

    assert_eq!(a.take_a(), 14);
}

#[test]
fn test_struct_single_unnamed() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub into_parts)]
    struct Test(
        #[quick_impl(
            pub const get,
            pub get_clone = "get_clone_0",
            get_mut,
            const into,
            pub(crate) set,
            pub take,
            pub(crate) const with,
            pub from,
            impl Deref,
            impl DerefMut,
            impl Into,
            impl From,
            impl AsRef,
            impl AsMut,
            impl Borrow,
            impl BorrowMut,
        )]
        usize,
    );

    let a = Test(12);
    assert_eq!(*a.get_0(), 12);
    assert_eq!(*<Test as Deref>::deref(&a), 12);

    let a = a.with_0(13);
    assert_eq!(a.into_0(), 13);

    let mut a = Test(12);
    assert_eq!(*<Test as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(a.set_0(13), 12);
    assert_eq!(*a.get_0_mut(), 13);

    *a.get_0_mut() = 14;
    assert_eq!(Into::<usize>::into(a.clone()), 14);

    assert_eq!(a.take_0(), 14);
}

#[test]
fn test_struct_generics_unnamed() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub into_parts)]
    struct Test<A, B>(
        #[quick_impl(
            pub get,
            pub get_clone = "get_clone_0",
            get_mut,
            into,
            pub(crate) set,
            pub take,
            pub(crate) with,
            pub from,
            impl Deref,
            impl DerefMut,
            impl AsRef,
            impl AsMut,
            impl From,
            impl Borrow,
            impl BorrowMut,
        )]
        A,
        B,
    );

    let a = Test(12usize, 12usize);
    assert_eq!(*a.get_0(), 12);
    assert_eq!(*<Test<usize, usize> as Deref>::deref(&a), 12);

    let a = a.with_0(13);
    assert_eq!(a.into_0(), 13);

    let mut a = Test(12usize, 12usize);
    assert_eq!(*<Test<usize, usize> as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(a.set_0(13), 12);
    assert_eq!(*a.get_0_mut(), 13);

    *a.get_0_mut() = 14;
    assert_eq!(Into::<usize>::into(a.clone()), 14);

    assert_eq!(a.take_0(), 14);
}

#[test]
fn test_struct_lifetimes() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub into_parts)]
    struct Test<'a>(
        #[quick_impl(
            pub const get,
            get_mut,
            const into,
            pub(crate) set,
            pub take,
            pub(crate) const with,
            pub from,
            impl Deref,
            impl DerefMut,
            impl Into,
            impl From,
            impl AsRef,
            impl AsMut,
            impl Borrow,
            impl BorrowMut,
        )]
        &'a usize,
    );

    let n = 12;
    let m = 13;

    let a = Test(&n);
    assert_eq!(**a.get_0(), 12);
    assert_eq!(**Deref::deref(&a), 12);

    let a = a.with_0(&m);
    assert_eq!(*a.into_0(), 13);

    let mut a = Test(&n);
    assert_eq!(**<Test as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(*a.set_0(&m), 12);
    assert_eq!(**a.get_0_mut(), 13);

    *a.get_0_mut() = &n;
    assert_eq!(*Into::<&usize>::into(a), 12);
}
