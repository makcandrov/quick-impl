use quick_impl::quick_impl_all;

#[test]
fn test_enum_all() {
    #[quick_impl_all(pub const is, pub const as_ref)]
    #[allow(unused)]
    enum Test {
        First(usize),
        #[quick_impl(pub try_into)]
        Second(isize),
        Third(char, usize),
        Fourth(String),
    }

    assert!(Test::First(12).is_first());
    assert!(Test::Second(12).is_second());
}
