# quick-impl

[<img alt="github" src="https://img.shields.io/badge/github-source-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/makcandrov/quick-impl)
[<img alt="crates.io" src="https://img.shields.io/crates/v/quick-impl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/quick-impl)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/quick-impl/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/quick-impl)

`quick-impl` is a Rust procedural macro that simplifies working with enums and structs by generating common methods and traits for each variant or field. This helps reduce boilerplate code and enhances the ergonomics of using enums and structs in your Rust projects.

## Usage

```rust
use quick_impl::QuickImpl;

#[derive(QuickImpl)]
enum YourEnum {
    #[quick_impl(pub const is)]
    Variant1,

    #[quick_impl(pub as_ref, pub(crate) as_ref_mut, impl From)]
    Variant2(i32),
}

fn main() {
    let instance = YourEnum::Variant1;

    // Use generated methods on enum instances
    assert!(instance.is_variant1());

    let variant2_instance = YourEnum::from(42);
    assert_eq!(*variant2_instance.as_variant2().unwrap(), 42);
}
```

More examples can be found in the [examples folder].

[examples folder]: https://github.com/makcandrov/quick-impl/tree/main/examples

## Features

### Enum variant methods

- `as_ref` - Returns an immutable reference to the associated data of the enum variant.
- `as_ref_mut` - Returns a mutable reference to the associated data of the enum variant.
- `from` - Creates an instance of the enum variant from the associated data.
- `into` - Converts the enum into the associated data of the variant, returning an [`Option`].
- `is` - Returns `true` if the enum matches the specified variant.
- `is_and` - Returns `true` if the enum matches the specified variant and the associated data matches a predicate.
- `set` - Replaces the current instance with a new instance of the specified variant.
- `try_into` - Converts the enum into the associated data of the variant, returning a [`Result`].

### Enum variant traits

- `Default` - Implements the [`Default`] trait on the enum.
- `From` - Implements the [`From`] trait on the enum.
- `TryInto` - Implements the [`TryInto`] trait on the enum.
- `TryFrom` - Implements the [`TryFrom`] trait on the associated data.

### Structure field methods

- `get` - A getter for the field. Returns a reference to the field.
- `get_clone` - A getter for the field. Returns a clone of the field.
- `get_mut` - A mutable getter for a field.
- `into` - Converts the struct into the field.
- `from` - Creates an instance from the field. Sets the other fields to their default value.
- `set` - A setter for the field.
- `take` - Returns the field and replaces it with its default value.
- `with` - Returns the struct with the field modified.

### Structure field traits

- `AsRef` - Implements the [`AsRef`] trait on the struct.
- `AsMut` - Implements the [`AsMut`] trait on the struct.
- `Borrow` - Implements the [`Borrow`] trait on the struct.
- `BorrowMut` - Implements the [`BorrowMut`] trait on the struct.
- `Deref` - Implements the [`Deref`] trait on the struct.
- `DerefMut` - Implements the [`DerefMut`] trait on the struct.
- `Into` - Implements the [`Into`] trait on the struct.
- `From` - Implements the [`From`] trait on the struct.

### Structure global methods

- `into_parts` - Destructures the instance into its fields values.
- `new` - Constructs a new instance from the specified field values.

[`AsRef`]: https://doc.rust-lang.org/core/convert/trait.AsRef.html
[`AsMut`]: https://doc.rust-lang.org/core/convert/trait.AsMut.html
[`Borrow`]: https://doc.rust-lang.org/core/borrow/trait.Borrow.html
[`BorrowMut`]: https://doc.rust-lang.org/core/borrow/trait.BorrowMut.html
[`Default`]: https://doc.rust-lang.org/core/default/trait.Default.html
[`Deref`]: https://doc.rust-lang.org/core/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/core/ops/trait.DerefMut.html
[`From`]: https://doc.rust-lang.org/core/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/core/convert/trait.Into.html
[`Option`]: https://doc.rust-lang.org/core/option/enum.Option.html
[`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html
[`TryFrom`]: https://doc.rust-lang.org/core/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/core/convert/trait.TryInto.html

## Installation

Add `quick-impl` to your `Cargo.toml`:

```toml
[dependencies]
quick-impl = "0.1"
```

Or run the following command:

```shell
cargo add quick-impl
```
