use std::f64::consts::PI;

use quick_impl::quick_impl;

#[quick_impl]
pub enum Shape {
    #[quick_impl(pub const is, pub inspect, impl Default)]
    Circle(f64),

    #[quick_impl(pub as_ref, as_ref_mut, pub set, impl From)]
    Rectangle(f64, f64),

    #[quick_impl(pub as_ref, as_ref_mut, pub into)]
    Square(f64),

    #[quick_impl(pub const from = "create_cuboid", pub const is, pub into)]
    Cuboid { width: f64, height: f64, depth: f64 },
}

fn main() {
    // `is` — check the variant
    let circle = Shape::Circle(5.0);
    assert!(circle.is_circle());

    // `inspect` — peek at the associated data without consuming
    let circle = Shape::Circle(PI).inspect_circle(|r| {
        assert_eq!(*r, PI);
    });
    assert!(circle.is_circle());

    // `impl Default` — creates the variant with default associated data
    let default_circle = Shape::default();
    assert!(default_circle.is_circle());

    // `impl From` — construct from the associated data
    let rectangle = Shape::from((3.0, 4.0));
    assert_eq!(rectangle.as_rectangle().unwrap(), (&3.0, &4.0));

    // `as_ref_mut` — mutate associated data in place
    let mut square = Shape::Square(10.0);
    *square.as_square_mut().unwrap() = 15.0;
    assert_eq!(*square.as_square().unwrap(), 15.0);

    // `into` — extract associated data, consuming `self`
    assert_eq!(square.into_square().unwrap(), 15.0);

    // `set` — replace the current variant, getting the previous value back
    let mut shape = Shape::Circle(1.0);
    let previous = shape.set_rectangle(5.0, 6.0);
    assert!(previous.is_circle());
    assert_eq!(shape.as_rectangle().unwrap(), (&5.0, &6.0));

    // `from` with custom name
    let cuboid = Shape::create_cuboid(2.0, 3.0, 4.0);
    assert!(cuboid.is_cuboid());

    let (width, height, depth) = cuboid.into_cuboid().unwrap();
    assert_eq!((width, height, depth), (2.0, 3.0, 4.0));
}
