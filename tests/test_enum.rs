use quick_impl::QuickImpl;

#[test]
fn test_enum_variant_unit() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryInto,
        )]
        A,
    }

    let a = Test::A;
    assert!(a.is_a());
    let _: () = a.as_a().unwrap();
    let _: () = a.into_a().unwrap();

    let mut a = Test::A;
    let _: () = a.as_a_mut().unwrap();

    assert_eq!(Test::from_a(), Test::A);
    assert_eq!(<Test as From<()>>::from(()), Test::A);
}

#[test]
fn test_enum_variant_single_unnamed() {
    #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
    enum Test {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom
        )]
        Variant1(usize),
    }

    let a = Test::Variant1(12);
    assert!(a.is_variant1());
    assert_eq!(*a.as_variant1().unwrap(), 12);
    assert_eq!(a.clone().into_variant1().unwrap(), 12);
    assert_eq!(a.clone().try_into_variant1().unwrap(), 12);
    assert_eq!(TryInto::<usize>::try_into(a.clone()).unwrap(), 12);

    let mut a = Test::Variant1(12);
    assert_eq!(*a.as_variant1_mut().unwrap(), 12);

    assert_eq!(Test::from_variant1(12), Test::Variant1(12));
    assert_eq!(<Test as From<usize>>::from(12), Test::Variant1(12));
}

#[test]
fn test_enum_variant_single_named() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryInto
        )]
        Variant1 { a: usize },
    }

    let a = Test::Variant1 { a: 12 };
    assert!(a.is_variant1());
    assert_eq!(*a.as_variant1().unwrap(), 12);
    assert_eq!(a.into_variant1().unwrap(), 12);

    let mut a = Test::Variant1 { a: 12 };
    assert_eq!(*a.as_variant1_mut().unwrap(), 12);

    assert_eq!(Test::from_variant1(12), Test::Variant1 { a: 12 });
    assert_eq!(<Test as From<usize>>::from(12), Test::Variant1 { a: 12 });
}

#[test]
fn test_enum_variant_multiple_unnamed() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryInto
        )]
        Variant1(usize, isize, char),
    }

    let a = Test::Variant1(12, -15, 'C');
    assert!(a.is_variant1());
    assert_eq!(a.as_variant1().unwrap(), (&12, &-15, &'C'));
    assert_eq!(a.into_variant1().unwrap(), (12, -15, 'C'));

    let mut a = Test::Variant1(12, -15, 'C');
    assert_eq!(a.as_variant1_mut().unwrap(), (&mut 12, &mut -15, &mut 'C'));

    assert_eq!(
        Test::from_variant1(12, -15, 'C'),
        Test::Variant1(12, -15, 'C')
    );
    assert_eq!(
        <Test as From<(usize, isize, char)>>::from((12, -15, 'C')),
        Test::Variant1(12, -15, 'C')
    );
}

#[test]
fn test_enum_variant_multiple_named() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            impl Default,
            impl From,
            impl TryInto
        )]
        Variant1 { a: usize, b: isize, c: char },
    }

    let a = Test::Variant1 {
        a: 12,
        b: -15,
        c: 'C',
    };
    assert!(a.is_variant1());
    assert_eq!(a.as_variant1().unwrap(), (&12, &-15, &'C'));
    assert_eq!(a.into_variant1().unwrap(), (12, -15, 'C'));

    let mut a = Test::Variant1 {
        a: 12,
        b: -15,
        c: 'C',
    };
    assert_eq!(a.as_variant1_mut().unwrap(), (&mut 12, &mut -15, &mut 'C'));

    assert_eq!(
        Test::from_variant1(12, -15, 'C'),
        Test::Variant1 {
            a: 12,
            b: -15,
            c: 'C'
        }
    );
    assert_eq!(
        <Test as From<(usize, isize, char)>>::from((12, -15, 'C')),
        Test::Variant1 {
            a: 12,
            b: -15,
            c: 'C'
        }
    );
}

#[test]
fn test_enum_generics() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test<T, U> {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom,
        )]
        Variant1 { a: T, b: U },
        #[allow(dead_code)]
        Variant2 { a: T, b: U },
    }

    let a = Test::Variant1 {
        a: 12usize,
        b: -15isize,
    };
    assert!(a.is_variant1());
    assert_eq!(a.as_variant1().unwrap(), (&12, &-15));
    assert_eq!(a.into_variant1().unwrap(), (12, -15));

    let mut a = Test::Variant1 {
        a: 12usize,
        b: -15isize,
    };
    assert_eq!(a.as_variant1_mut().unwrap(), (&mut 12, &mut -15));

    assert_eq!(
        Test::from_variant1(12, -15),
        Test::Variant1 { a: 12, b: -15 }
    );
    assert_eq!(
        <Test<usize, isize> as From<(usize, isize)>>::from((12, -15)),
        Test::Variant1 { a: 12, b: -15 }
    );
}

#[test]
fn test_enum_lifetimes() {
    #[derive(Debug, Eq, PartialEq, QuickImpl)]
    enum Test<'a, 'b> {
        #[quick_impl(
            pub(crate) is,
            pub(self) is_and,
            as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom
        )]
        Variant1(&'a usize, &'b mut isize),
    }

    let mut s1 = -15;
    let mut s2 = -15;
    let mut s3 = -15;
    let mut s4 = -15;

    let a = Test::Variant1(&12, &mut s1);
    assert!(a.is_variant1());
    assert_eq!(a.as_variant1().unwrap(), (&&12, &&mut s2));
    assert_eq!(a.into_variant1().unwrap(), (&12, &mut s2));

    let mut a = Test::Variant1(&12, &mut s2);
    assert_eq!(a.as_variant1_mut().unwrap(), (&mut &12, &mut &mut s3));

    assert_eq!(
        Test::from_variant1(&12, &mut -15),
        Test::Variant1(&12, &mut s3)
    );
    assert_eq!(
        <Test as From<(&usize, &mut isize)>>::from((&12, &mut s3)),
        Test::Variant1(&12, &mut s4)
    );
}

#[test]
fn test_empty_enums() {
    #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
    enum TestA {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom
        )]
        A,
    }

    #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
    enum TestB {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom
        )]
        B(),
    }

    #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
    enum TestC {
        #[quick_impl(
            pub(crate) const is,
            pub(self) is_and,
            const as_ref,
            pub as_ref_mut,
            pub(crate) from,
            pub(crate) into,
            set,
            pub try_into,
            pub inspect,
            impl Default,
            impl From,
            impl TryFrom
        )]
        C {},
    }

    let _: () = TestA::A.into_a().unwrap();
    let _: () = TestB::B().into_b().unwrap();
    let _: () = TestC::C {}.into_c().unwrap();
}

#[test]
fn test_variable_rename() {
    {
        #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
        enum TestA {
            #[quick_impl(pub is_and, pub inspect)]
            Variant1 { a: usize, b: usize, c: usize },
            #[allow(unused)]
            Variant2 { a: usize, b: usize, c: usize },
        }

        let mut called = false;

        let instance = TestA::Variant1 { a: 1, b: 2, c: 3 }.inspect_variant1(|a, b, c| {
            called = true;
            assert_eq!((*a, *b, *c), (1, 2, 3));
        });

        assert!(called);
        assert!(instance.is_variant1_and(|a, _, _| *a == 1));
    }

    {
        #[derive(Debug, Clone, Eq, PartialEq, QuickImpl)]
        enum TestF {
            #[quick_impl(pub is_and, pub inspect)]
            Variant1 { f: usize, b: usize, c: usize },
            #[allow(unused)]
            Variant2 { f: usize, b: usize, c: usize },
        }

        let mut called = false;

        let instance = TestF::Variant1 { f: 1, b: 2, c: 3 }.inspect_variant1(|f, b, c| {
            called = true;
            assert_eq!((*f, *b, *c), (1, 2, 3));
        });

        assert!(called);
        assert!(instance.is_variant1_and(|a, _, _| *a == 1));
    }
}
