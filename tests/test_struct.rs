use std::ops::{Deref, DerefMut};

use quick_impl::QuickImpl;

#[test]
fn test_struct_single_named() {
    #[derive(QuickImpl)]
    struct Test {
        #[quick_impl(pub const get = "{}", pub get_clone, get_mut, const into, pub(crate) set, pub(crate) const with, impl Deref, impl DerefMut, impl Into, impl AsRef, impl AsMut)]
        a: usize,
    }

    let a = Test { a: 12 };
    assert_eq!(*a.a(), 12);
    assert_eq!(*<Test as Deref>::deref(&a), 12);

    let a = a.with_a(13);
    assert_eq!(a.into_a(), 13);

    let mut a = Test { a: 12 };
    assert_eq!(*<Test as DerefMut>::deref_mut(&mut a), 12);

    assert_eq!(a.set_a(13), 12);
    assert_eq!(*a.get_a_mut(), 13);

    *a.get_a_mut() = 14;
    assert_eq!(Into::<usize>::into(a), 14)
}

#[test]
fn test_struct_single_unnamed() {
    #[derive(QuickImpl)]
    struct Test(
        #[quick_impl(pub const get, get_mut, const into, pub(crate) set, pub(crate) const with, impl Deref, impl DerefMut, impl Into, impl AsRef, impl AsMut)]
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
    assert_eq!(Into::<usize>::into(a), 14)
}
