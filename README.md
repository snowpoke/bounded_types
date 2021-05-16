# bounded_types

Provides newtypes `BoundedI32`, `BoundedI64`, etc. which behave similar to their raw counterparts, but guarantee that the value is within a range that you specify.
In contrast to other crates like this, these types are implemented using the newly stabilized const generics feature, which allows for simplifications that make the use of this type more intuitive and idiomatic.

They are wrappers around a `Result`, but implement traits like `PartialEq<{Integer}>` and even `Ord<{Integer}>` that make them act like integers in many ways. Some traits (like `Add`, for example) are intentionally not implemented, since those would be invalid on out-of-bounds values.

## Example

 ```rust
use bounded_types::BoundedI64;

// If an in-bounds value is stored, comparisons behave like you would expect.
let bounded_ok: BoundedI64<2, 10> = 5.into();

assert!(bounded_ok == 5);
assert!(bounded_ok >= 5);
assert!(bounded_ok >= 4);
// you can compare with any integer
assert!(bounded_ok < 100);
assert!(bounded_ok > -100);

// If an out-of-bounds value is stored, comparisons always return `false`
let bounded_err: BoundedI64<2, 10> = 11.into();

assert_eq!(bounded_err == 11, false);
assert_eq!(bounded_err > 5, false);
```

## Memory use

```rust
use bounded_types::*;
use std::mem::size_of;
assert!(size_of::<Option<i8>>() == size_of::<BoundedI8<0, 10>>());
assert!(size_of::<Option<i16>>() == size_of::<BoundedI16<0, 10>>());
assert!(size_of::<Option<i32>>() == size_of::<BoundedI32<0, 10>>());
assert!(size_of::<Option<i64>>() == size_of::<BoundedI64<0, 10>>());
assert!(size_of::<Option<i128>>() == size_of::<BoundedI128<0, 10>>());
// etc. you get the idea
```

## License

`bounded_types` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.