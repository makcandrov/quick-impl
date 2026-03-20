# quick-impl

[![Crates.io](https://img.shields.io/crates/v/quick-impl)](https://crates.io/crates/quick-impl)
[![docs.rs](https://img.shields.io/docsrs/quick-impl)](https://docs.rs/quick-impl)
[![License](https://img.shields.io/crates/l/quick-impl)](https://github.com/makcandrov/quick-impl#license)

`quick-impl` is a Rust procedural macro that generates usual methods and trait implementations for enums and structs, reducing boilerplate and improving ergonomics.

## Quick start

```rust
use quick_impl::quick_impl;

#[quick_impl]
enum Shape {
    #[quick_impl(pub const is)]
    Circle(f64),

    #[quick_impl(pub as_ref, as_ref_mut, impl From)]
    Rectangle(f64, f64),

    #[quick_impl(pub as_ref, pub into)]
    Square(f64),
}

let circle = Shape::Circle(5.0);
assert!(circle.is_circle());

let rect = Shape::from((3.0, 4.0));
assert_eq!(rect.as_rectangle().unwrap(), (&3.0, &4.0));

let square = Shape::Square(10.0);
assert_eq!(square.into_square().unwrap(), 10.0);
```

More examples can be found in the [examples folder].

[examples folder]: https://github.com/makcandrov/quick-impl/tree/main/examples

## Features

### Enum variant methods

| Attribute   | Description                                                                                         |
|-------------|-----------------------------------------------------------------------------------------------------|
| `as_ref`    | Returns a reference to the associated data if the enum is the specified variant.                    |
| `as_ref_mut`| Returns a mutable reference to the associated data if the enum is the specified variant.            |
| `from`      | Creates the specified variant from the provided data.                                               |
| `inspect`   | Calls a closure with a reference to the associated data if the enum is the specified variant, and returns `self`. |
| `into`      | Extracts the associated data if the enum is the specified variant, returning an [`Option`].         |
| `is`        | Returns `true` if the enum is the specified variant.                                                |
| `is_and`    | Returns `true` if the enum is the specified variant and the associated data satisfies a predicate.  |
| `set`       | Sets `self` to the specified variant with the given data, returning the previous value.             |
| `try_into`  | Extracts the associated data if the enum is the specified variant, returning a [`Result`].          |

### Enum variant traits

| Attribute  | Description                                                      |
|------------|------------------------------------------------------------------|
| `Default`  | Implements [`Default`], constructing the specified variant.      |
| `From`     | Implements [`From`] to construct the specified variant.          |
| `TryFrom`  | Implements [`TryFrom`] to extract the associated data.           |
| `TryInto`  | Implements [`TryInto`] to extract the associated data.           |

### Struct field methods

| Attribute | Description                                                                             |
|-----------|-----------------------------------------------------------------------------------------|
| `get`     | Returns a reference to the field.                                                       |
| `get_clone` | Returns a clone of the field.                                                         |
| `get_mut` | Returns a mutable reference to the field.                                               |
| `into`    | Consumes `self` and returns the field.                                                  |
| `from`    | Creates an instance from the field, setting the remaining fields to their default values.|
| `replace` | Replaces the field with the given value, returning the previous value.                  |
| `set`     | Sets the field and returns `&mut self` for chaining.                                    |
| `take`    | Takes the field, replacing it with its default value.                                   |
| `with`    | Returns `self` with the field set to the given value.                                   |

### Struct field traits

| Attribute   | Description                                                                               |
|-------------|-------------------------------------------------------------------------------------------|
| `AsRef`     | Implements [`AsRef`] to return a reference to the field.                                  |
| `AsMut`     | Implements [`AsMut`] to return a mutable reference to the field.                          |
| `Borrow`    | Implements [`Borrow`] for the field type.                                                 |
| `BorrowMut` | Implements [`BorrowMut`] for the field type.                                              |
| `Deref`     | Implements [`Deref`] with the field as the target.                                        |
| `DerefMut`  | Implements [`DerefMut`] with the field as the target.                                     |
| `From`      | Implements [`From`] to create the struct from the field, defaulting the remaining fields. |
| `Into`      | Implements [`Into`] to convert the struct into the field value.                           |

### Struct global methods

| Attribute    | Description                                                      |
|--------------|------------------------------------------------------------------|
| `new`        | Constructs a new instance from the given field values.           |
| `from_tuple` | Constructs a new instance from a tuple of field values.          |
| `into_parts` | Decomposes the instance into a tuple of its field values.        |

### Struct global traits

| Attribute | Description                                                                  |
|-----------|------------------------------------------------------------------------------|
| `From`    | Implements [`From`] to create the struct from a tuple of its field values.   |
| `Into`    | Implements [`Into`] to convert the struct into a tuple of its field values.  |

## Reducing duplication with `quick_impl_all`

`quick_impl_all` applies the specified attributes to all variants (for enums) or all fields (for structs). You can combine it with per-variant/per-field `quick_impl` attributes:

```rust
use quick_impl::quick_impl_all;

#[quick_impl_all(pub const is)]
enum MyEnum {
    VariantA,

    #[quick_impl(pub from)]
    VariantB(i32),
}

fn main() {
    let variant = MyEnum::from_variant_b(10);
    assert!(variant.is_variant_b());
    assert!(!variant.is_variant_a());
}
```

## Configuration

### Method configuration

- **`name`** — Override the generated method name. Use `{}` as a placeholder for the variant/field name.

```rust
#[quick_impl::quick_impl]
struct Foo {
    #[quick_impl(pub get_clone = { name = "get_{}_unchecked" })]
    bar: usize,
    #[quick_impl(pub get_clone = "get_{}_unchecked")] // shorthand
    baz: usize,
}

fn main() {
    let foo = Foo { bar: 1, baz: 2 };
    assert_eq!(foo.get_bar_unchecked(), 1);
    assert_eq!(foo.get_baz_unchecked(), 2);
}
```

- **`doc`** — Override the generated documentation string.

```rust
#[quick_impl::quick_impl(pub const new = { doc = "Generates an awesome instance of [`Foo`]." })]
struct Foo {
    bar: usize,
    baz: usize,
}
```

### Trait configuration

- **`doc`** — Override the generated documentation for the trait method.

```rust
#[quick_impl::quick_impl]
enum Foo {
    #[quick_impl(impl TryFrom = { doc = "Attempts to extract the data from a [`Foo::Bar`] variant." })]
    Bar(usize),
    Baz(isize),
}
```

## Comparison with `derive_more`

This crate is not intended to replace [`derive_more`]. While [`derive_more`] focuses on deriving standard traits, `quick-impl` focuses on generating common methods like `is_*`, `as_*`, and `set_*`. Trait implementations are included where they complement the method generation, but matching the breadth of [`derive_more`] is a non-goal.

[`derive_more`]: https://crates.io/crates/derive_more

## Installation

Add `quick-impl` to your `Cargo.toml`:

```toml
[dependencies]
quick-impl = "0.2"
```

Or run:

```shell
cargo add quick-impl
```

## License

Licensed under either of [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT License](https://opensource.org/licenses/MIT), at your option.

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
