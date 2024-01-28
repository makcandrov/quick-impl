use std::ops::Deref;

use quick_impl::QuickImpl;

#[test]
fn test_struct_multiple_named() {
    #[derive(QuickImpl)]
    struct Test {
        #[quick_impl(impl Deref)]
        a: usize,
    }

    let a = Test { a: 12 };
    assert_eq!(*<Test as Deref>::deref(&a), 12);
}
