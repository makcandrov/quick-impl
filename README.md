# quick-impl

`quick-impl` is a Rust procedural macro that simplifies working with enums and structures by generating common methods and traits for each variant/field. This helps reduce boilerplate code and enhances the ergonomics of using enums and structures in your Rust projects.

## Features

### Enums methods

- `as_ref` - Returns an immutable reference to the associated data of the enum variant.
- `as_ref_mut` - Returns a mutable reference to the associated data of the enum variant.
- `from` - Creates an instance of the enum variant from the associated data.
- `into` - Converts the enum into the variant associated data, returning an [`Option`].
- `is` - Checks if the enum variant matches a specified variant.
- `try_into` - Converts the enum into the variant associated data, returning a [`Result`].

### Enums traits

- `Default` - Implements the [`Default`] trait.
- `From` - Implements the [`From`] trait.
- `TryInto` - Implements the [`TryInto`] trait.

### Structures methods

- `get` - A getter for the field. Returns a reference to the field.
- `get_clone` - A getter for the field. Returns a clone of the field.
- `get_mut` - A mutable getter for the field.
- `into` - Converts the structure into the field.
- `set` - A setter for the field.
- `take` - Returns the field and replaces it with its default value.
- `with` - Returns the sutrcture with the field modified.

### Structures traits

- `AsRef` - Implements the [`AsRef`] trait.
- `AsMut` - Implements the [`AsMut`] trait.
- `Deref` - Implements the [`Deref`] trait.
- `DerefMut` - Implements the [`DerefMut`] trait.
- `Into` - Implements the [`Into`] trait.

[`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html

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
    #[quick_impl(pub const is)]
    Variant1,
    #[quick_impl(pub as_ref, pub(crate) as_ref_mut, impl From)]
    Variant2(i32),
    // ... add attributes to other variants as needed
}

fn main() {
    let instance = YourEnum::Variant1;

    // Use generated methods on enum instances
    assert!(instance.is_variant1());

    let variant2_instance = YourEnum::from(42);
    assert_eq!(*variant2_instance.as_variant2().unwrap(), 42);
}
```

More examples can be found in [examples].

[examples]: https://github.com/makcandrov/quick-impl/tree/main/examples
