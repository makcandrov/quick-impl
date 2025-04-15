use core::ops::{Deref, DerefMut};

use quick_impl::QuickImpl;

#[test]
fn test_struct_single_named() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test {
        #[quick_impl(
            pub const get = "{}",
            pub get_clone,
            get_mut,
            const into,
            pub(crate) replace,
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

    assert_eq!(a.set_a(11).a, 11);
    assert_eq!(a.replace_a(13), 11);
    assert_eq!(*a.get_a_mut(), 13);

    *a.get_a_mut() = 14;
    assert_eq!(Into::<usize>::into(*a), 14);

    assert_eq!(a.take_a(), 14);
}

#[test]
fn test_struct_single_unnamed() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test(
        #[quick_impl(
            pub const get,
            pub get_clone = "get_clone_0",
            get_mut,
            const into,
            replace,
            pub(crate) set,
            pub take,
            pub(crate) const with,
            pub from,
            impl Deref,
            impl DerefMut,
            // impl Into,
            // impl From,
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

    assert_eq!(a.replace_0(13), 12);
    assert_eq!(*a.get_0_mut(), 13);

    *a.get_0_mut() = 14;
    assert_eq!(Into::<usize>::into(*a), 14);

    assert_eq!(a.take_0(), 14);

    let Test(_) = Test::new(0);
    let Test(_) = Test::from_tuple((0,));
    let Test(_) = Test::from(0);
    let Test(_) = Test::from((0,));
    assert_eq!(usize::from(Test(1)), 1usize);
    assert_eq!(<(usize,)>::from(Test(1)), (1usize,));
    assert_eq!(Test(1).into_parts(), 1usize);
}

#[test]
fn test_struct_generics_unnamed() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub from_tuple, pub into_parts, impl From, impl Into)]
    struct Test<A, B>(
        #[quick_impl(
            pub get,
            pub get_clone = "get_clone_0",
            get_mut,
            into,
            replace,
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

    assert_eq!(a.replace_0(13), 12);
    assert_eq!(*a.get_0_mut(), 13);

    *a.get_0_mut() = 14;
    assert_eq!(Into::<usize>::into(*a), 14);

    assert_eq!(a.take_0(), 14);
}

#[test]
fn test_struct_lifetimes() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test<'a, 'b>(
        #[quick_impl(
            pub const get,
            get_mut,
            const into,
            replace,
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
        &'b i32,
    );

    let n = 12;
    let m = 13;

    let a = Test(&n, &12);
    assert_eq!(**a.get_0(), 12);
    assert_eq!(**Deref::deref(&a), 12);

    let a = a.with_0(&m);
    assert_eq!(*a.into_0(), 13);

    let mut a = Test(&n, &12);
    assert_eq!(**<Test as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(*a.replace_0(&m), 12);
    assert_eq!(**a.get_0_mut(), 13);

    *a.get_0_mut() = &n;
    assert_eq!(*Into::<&usize>::into(a), 12);
}

#[test]
fn test_empty_struct() {
    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test1;

    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test2 {}

    #[derive(QuickImpl)]
    #[quick_impl(pub const new, pub const from_tuple, pub into_parts, impl From, impl Into)]
    struct Test3();

    let Test1 = Test1::new();
    let Test1 = Test1::from_tuple(());
    let Test1 = Test1::from(());
    let () = Test1.into_parts();
    let () = Test1.into();

    let Test2 {} = Test2::new();
    let Test2 {} = Test2::from_tuple(());
    let Test2 {} = Test2::from(());
    let () = Test2 {}.into_parts();
    let () = Test2 {}.into();

    let Test3() = Test3::new();
    let Test3() = Test3::from_tuple(());
    let Test3() = Test3::from(());
    let () = Test3().into_parts();
    let () = Test3().into();
}
