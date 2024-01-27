# quick-impl

`quick-impl` is a Rust procedural macro that simplifies working with enums by generating common methods and traits for each variant. This helps reduce boilerplate code and enhances the ergonomics of using enums in your Rust projects.

## Features

- `[pub] as_ref [= "rename"]` Generates a method that returns an immutable reference to the associated data of the enum variant.
- `[pub] as_ref_mut [= "rename"]` Generates a method that returns a mutable reference to the associated data of the enum variant.
- `[pub] from [= "rename"]` Generates a method that creates an instance of the enum variant from the associated data.
- `impl from` Implements the [`From`] trait for the enum, creating an instance of the enum variant from the associated data.
- `[pub] into [= "rename"]` Generates a method that converts the enum into the variant associated data.
- `[pub] is [= "rename"]` Generates a method that returns a boolean indicating whether the enum instance matches the specified variant.

[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html

## Usage

Add `quick-impl` to your `Cargo.toml`:

```toml
[dependencies]
quick-impl = "0.1"
```

In your Rust code:

```rust
use quick_impl::QuickImpl;

#[derive(QuickImpl)]
enum YourEnum {
    #[quick_impl(pub is)]
    Variant1,
    #[quick_impl(pub as_ref, as_ref_mut, impl from)]
    Variant2(i32),
    // ... add attributes to other variants as needed
}

fn main() {
    let instance = YourEnum::Variant1;

    // Use generated methods on enum instances
    assert!(instance.is_variant_1());

    let variant2_instance = YourEnum::from(42);
    assert_eq!(*variant2_instance.as_variant_2().unwrap(), 42);
}
```

More examples can be found in [examples].

[examples]: https://github.com/makcandrov/quick-impl/tree/main/examples
