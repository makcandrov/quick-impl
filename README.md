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
    let instance1 = YourEnum::Variant1;

    assert!(instance1.is_variant1());

    let instance2 = YourEnum::from(42);
    assert_eq!(*instance2.as_variant2().unwrap(), 42);
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

- `AsRef` - Implements the [`AsRef`] trait for the struct.
- `AsMut` - Implements the [`AsMut`] trait for the struct.
- `Borrow` - Implements the [`Borrow`] trait for the struct.
- `BorrowMut` - Implements the [`BorrowMut`] trait for the struct.
- `Deref` - Implements the [`Deref`] trait for the struct.
- `DerefMut` - Implements the [`DerefMut`] trait for the struct.
- `From` - Implements the [`From`] trait for the struct, allowing it to be created from the field value while setting the other fields to their default values.
- `Into` - Implements the [`Into`] trait for the struct, converting the structure instance into the field value.

### Structure global methods

- `into_parts` - Destructures the instance into its fields values.
- `new` - Constructs a new instance from the given field values.

### Structure global traits

- `From` - Implements the [`From`] trait for the struct, allowing it to be created from a tuple of its field values.
- `Into` - Implements the [`Into`] trait for the struct, converting the structure instance into a tuple of its field values.

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

## Configuration

### Method configurations

- `name` - Sets the name of the generated method. If not set, a default name is used.

```rust
#[derive(quick_impl::QuickImpl)]
struct Foo {
    #[quick_impl(pub get_clone = { name = "get_{}_unchecked"})]
    bar: usize,
    #[quick_impl(pub get_clone = "get_{}_unchecked")] // Shorter version
    baz: usize,
}

let instance = Foo { bar: 1, baz: 2 };

assert_eq!(instance.get_bar_unchecked(), 1);
assert_eq!(instance.get_baz_unchecked(), 2);
```

- `doc` - Sets the documentation for the generated method. If not set, a default documentation is generated.

```rust
#[derive(quick_impl::QuickImpl)]
#[quick_impl(pub const new = { doc = "Generates an awesome instance of [`Foo`]." })]
struct Foo {
    bar: usize,
    baz: usize,
}
```

### Traits configurations

- `doc` - Sets the documentation of the generated trait method. If not set, a default documentation is generated.

```rust
#[derive(quick_impl::QuickImpl)]
enum Foo {
    #[quick_impl(impl TryFrom = { doc = "Attempts to extract the associated data from a [`Foo::Bar`] variant." })]
    Bar(usize),
    Baz(isize),
}
```

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
