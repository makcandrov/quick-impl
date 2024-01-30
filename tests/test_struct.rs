use std::ops::{Deref, DerefMut};

use quick_impl::QuickImpl;

#[test]
fn test_struct_single_named() {
    #[derive(QuickImpl)]
    struct Test {
        #[quick_impl(pub const get, impl Deref, impl DerefMut)]
        a: usize,
    }

    let a = Test { a: 12 };
    assert_eq!(*a.get_a(), 12);
    assert_eq!(*<Test as Deref>::deref(&a), 12);

    let mut a = Test { a: 12 };
    assert_eq!(*<Test as DerefMut>::deref_mut(&mut a), 12);
}

#[test]
fn test_struct_single_unnamed() {
    #[derive(QuickImpl)]
    struct Test(#[quick_impl(pub const get, impl Deref, impl DerefMut)] usize);

    let a = Test(12);
    assert_eq!(*a.get_0(), 12);
    assert_eq!(*<Test as Deref>::deref(&a), 12);

    let mut a = Test(12);
    assert_eq!(*<Test as DerefMut>::deref_mut(&mut a), 12);
}
