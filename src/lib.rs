/*! Provides newtypes `BoundedI32`, `BoundedI64`, etc. which behave similar to their raw counterparts, but guarantee that the value is within a range that you specify.
In contrast to other crates like this, these types are implemented using the newly stabilized const generics feature, which allows for simplifications that make the use of this type more intuitive and idiomatic.

They are wrappers around a `Result`, but implement traits like `PartialEq<{Integer}>` and even `Ord<{Integer}>` that make them act like integers in many ways. Some traits (like `Add`, for example) are intentionally not implemented, since those would be invalid on out-of-bounds values.

## Example
 ```
use bounded_types::BoundedI64;

// If an in-bounds value is stored, comparisons behave like you would expect.
let bounded_ok: BoundedI64<2, 10> = 5.into();

assert!(bounded_ok == 5);
assert!(bounded_ok >= 5);
assert!(bounded_ok >= 4);
// you can compare with any integer
assert!(bounded_ok < 100);
assert!(bounded_ok > -100);

// If an out-of-bounds value is stored, equal-checks and unequality-checks always return `false`. unequal-checks return true because ne() has to be the inverse of eq().
let bounded_err: BoundedI64<2, 10> = 11.into();

assert_eq!(bounded_err == 11, false);
assert_eq!(bounded_err != 11, true);
assert_eq!(bounded_err > 5, false);
```

## Memory use
```
use bounded_types::*;
use std::mem::size_of;
assert!(size_of::<Option<i8>>() == size_of::<BoundedI8<0, 10>>());
assert!(size_of::<Option<i16>>() == size_of::<BoundedI16<0, 10>>());
assert!(size_of::<Option<i32>>() == size_of::<BoundedI32<0, 10>>());
assert!(size_of::<Option<i64>>() == size_of::<BoundedI64<0, 10>>());
assert!(size_of::<Option<i128>>() == size_of::<BoundedI128<0, 10>>());
// etc. you get the idea
```
*/
#![deny(
    deprecated_in_future,
    exported_private_dependencies,
    future_incompatible,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    private_in_public,
    rust_2018_compatibility,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_lifetimes,
    variant_size_differences
)]
#![warn(clippy::pedantic)]

pub use crate::i128::BoundedI128;
pub use crate::i16::BoundedI16;
pub use crate::i32::BoundedI32;
pub use crate::i64::BoundedI64;
pub use crate::i8::BoundedI8;
pub use crate::isize::BoundedIsize;

pub use crate::u128::BoundedU128;
pub use crate::u16::BoundedU16;
pub use crate::u32::BoundedU32;
pub use crate::u64::BoundedU64;
pub use crate::u8::BoundedU8;
pub use crate::usize::BoundedUsize;

#[macro_use]
/// Derives traits that define the relation to other numeric types. Like From, PartialEq, PartialOrd.
macro_rules! derive_numeric_traits {
    ( $type: ident, $bound:ty, $int:ty; $( $numeric:ty ),* )  => {
        $(

        impl<const MIN: $bound, const MAX: $bound> From<$numeric> for $type<MIN, MAX> {
            fn from(other: $numeric) -> Self {
                match <$int>::try_from(other) {
                    Ok(this) => {
                        if Self::is_in_bounds(&this) {
                            Self(Ok(this))
                        } else {
                            Self::out_of_bounds(this)
                        }
                    }
                    // if we try to cast from a number that cannot be parsed as $int, we save $int::MAX as error value
                    Err(_) => Self::out_of_bounds(<$int>::MAX),
                }
            }
        }

        // impl Add<$numeric> for Unbounded<UnboundedVal, $int, Bound> {
        //     type Output = Unbounded<UnboundedVal, $int, Bound>;
        //     fn add(self, other: $numeric) -> Self::Output {
        //         #[allow(trivial_numeric_casts)]
        //         match self.0 {
        //             Ok(self_val) => Ok(self_val + (other as UnboundedVal)).into(),
        //             Err((self_carryover, self_errvec)) => Err((self_carryover + (other as UnboundedVal), self_errvec)).into()
        //         }
        //     }
        // }

        // /// Deduced through Add<$numeric> for Unbounded.
        // impl<const MIN: Bound, const MAX: Bound> Add<$numeric> for $type<MIN, MAX> {
        //     type Output = Unbounded<UnboundedVal, $int, Bound>;
        //     #[allow(trivial_numeric_casts)]
        //     fn add(self, other: $numeric) -> Self::Output {
        //         self.into_unbounded() + (other as UnboundedVal)
        //     }
        // }


        // /// Deduced through symmetry.
        // impl<const MIN: Bound, const MAX: Bound> Add<$type<MIN, MAX>> for $numeric {
        //     type Output = Unbounded<UnboundedVal, $int, Bound>;
        //     #[allow(trivial_numeric_casts)]
        //     fn add(self, other: $type<MIN, MAX>) -> Self::Output {
        //         other + self
        //     }
        // }


        impl<const MIN: $bound, const MAX: $bound> PartialEq<$numeric> for $type<MIN, MAX> {
            // will throw false if values don't match or Numeric can't be cast as $int
            fn eq(&self, other: &$numeric) -> bool {
                match (&self.0, <$int>::try_from(*other)) {
                    (&Ok(val), Ok(other)) => val == other,
                    _ => false,
                }
            }
        }

        impl<const MIN: $bound, const MAX: $bound> PartialOrd<$numeric> for $type<MIN, MAX> {
            // will throw false if values don't match or Numeric can't be cast as $int
            fn partial_cmp(&self, other: &$numeric) -> Option<Ordering> {
                match (&self.0, <$int>::try_from(*other)) {
                    (Ok(self_val), Ok(other_val)) => self_val.partial_cmp(&other_val),
                    _ => None,
                }
            }
        }

        /// Inferred through symmetry.
        impl<const MIN: $bound, const MAX: $bound> PartialEq<$type<MIN, MAX>> for $numeric {
            fn eq(&self, other: &$type<MIN, MAX>) -> bool {
                other == self
            }
        }

        /// Inferred through assymetry.
        impl<const MIN: $bound, const MAX: $bound> PartialOrd<$type<MIN, MAX>> for $numeric {
            fn partial_cmp(&self, other: &$type<MIN, MAX>) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }

    )*
    };
}

// /// Numeric type stored within Unbounded, the type produced after operations are performed on `BoundedI64` elements. This should be larger or equal in size to Int.
// /// Int = `UnboundedVal` seems natural for Int = i32, but for Int = usize, you might want `UnboundedVal` to be larger (like i128), so Int and `UnboundedVal` are separate.
// type UnboundedVal = i64;

/// Generates a bounded type with the specified type name, bound type and value type.
#[macro_use]
macro_rules! generate_type {
    ( $type: ident, $bound:ty, $int:ty )   => {
        use derive_more::Constructor;
        use shrinkwraprs::Shrinkwrap; //derives Deref, Borrow and AsRef
        use std::cmp::Ordering;
        use std::cmp::{PartialEq, PartialOrd};
        use std::convert::TryFrom;
        use std::fmt::Debug;
        use std::str::FromStr;

#[derive(Shrinkwrap, Constructor)]
/// The error that is returned when you attempt to assign an out-of-bounds value to a bounded type. This is stored as pointer so that enums containing it won't take up too much space.
pub struct OutOfBoundsError<const MIN: $bound, const MAX: $bound>($int);


impl<const MIN: $bound, const MAX: $bound> OutOfBoundsError<MIN, MAX> {
    /// Returns a reference to the value that was attempted to be passed.
    #[must_use]
    pub fn value(&self) -> $int {
        self.0
    }

    /// Returns a reference to the smallest allowed value.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn min_allowed(&self) -> $bound {
        MIN
    }

    /// Returns a reference to the largest allowed value.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn max_allowed(&self) -> $bound {
        MAX
    }
}

impl<const MIN: $bound, const MAX: $bound> Debug for OutOfBoundsError<MIN, MAX> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OutOfBoundsError")
            .field("value", &self.value())
            .field("min_allowed", &self.min_allowed())
            .field("max_allowed", &self.max_allowed())
            .finish()
    }
}

paste::paste!{
#[derive(Shrinkwrap, Debug)]
#[doc="An `" $int "` element that is forced to be within the inclusive range `MIN..=MAX`."]
pub struct $type<const MIN: $bound, const MAX: $bound>(
    Result<$int, OutOfBoundsError<MIN, MAX>>,
);
}


// /// A compound error type that stores result and errors of multiple operations between bounded values.
// type MultiOutOfBoundsError<S, B> = (UnboundedVal, Vec<OutOfBoundsError<S, B>>);

// #[derive(Shrinkwrap, From, Debug)]
// /// An unbounded data type that bounded data types are converted into after operations are performed on them. Currently not implemented, bounded types will have to be unwrapped before operating on them.
// /// If the original data is Err(_), it will also be Err(_). Furthermore, if the Unbounded element is the result of an operation between multiple bounded data types, and at least one of them is Err(_), it stores the out of bounds errors of all original elements in a vector. The error type also holds the result that the operation would have if bounds were ignored.
// struct Unbounded<T, S: Debug, B: Debug>(Result<T, MultiOutOfBoundsError<S, B>>);

impl<const MIN: $bound, const MAX: $bound> $type<MIN, MAX> {
    /// Returns the numeric value stored in the struct, but overrides the bounds check.
    #[must_use]
    pub fn unchecked(&self) -> $int {
        match &self.0 {
            Ok(val) => *val,
            Err(err) => err.value(),
        }
    }

    /// Transforms bounded $int into an unbounded $int of Unbounded<> type.
    // #[must_use]
    // #[allow(trivial_numeric_casts)]
    // pub fn into_unbounded(self) -> Unbounded<UnboundedVal, $int, Bound> {
    //     self.0
    //         .map(|val| val as UnboundedVal)
    //         .map_err(|err| {
    //             (
    //                 (*err).0 as UnboundedVal, // store attempted value as carry-over value in MultiOutOfBoundsError
    //                 vec![err],
    //             )
    //         })
    //         .into()
    // }

    /// Returns an out of bounds error after a failed conversion.
    fn out_of_bounds(val: $int) -> Self {
        Self(Err(OutOfBoundsError::new(val)))
    }

    /// Function that returns whether a value is within the bounds.
    pub fn is_in_bounds(val: &impl PartialOrd<$int>) -> bool {
        *val >= MIN && *val <= MAX
    }
}

impl<const MIN: $bound, const MAX: $bound> std::fmt::Display for $type<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Ok(val) => write!(f, "{}", val),
            Err(err) => write!(f, "{:?}", err),
        }
    }
}

impl<const MIN: $bound, const MAX: $bound> FromStr for $type<MIN, MAX> {
    type Err = <$bound as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <$bound>::from_str(s).map($type::from)
    }
}

impl<const MIN: $bound, const MAX: $bound> PartialEq for $type<MIN, MAX> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Ok(self_val), Ok(other_val)) => self_val == other_val,
            _ => false,
        }
    }
}

// Note: $type is not Eq because x != x for x.is_err()

impl<const MIN: $bound, const MAX: $bound> PartialOrd for $type<MIN, MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (Ok(self_val), Ok(other_val)) => self_val.partial_cmp(other_val),
            _ => None,
        }
    }
}
// // allowing for error-less conversion from Unbounded -> Bounded risks us ignoring errors, so we only allow try_into().
// impl<const MIN: Bound, const MAX: Bound> TryFrom<Unbounded<UnboundedVal, $int, Bound>>
//     for $type<MIN, MAX>
// {
//     type Error = MultiOutOfBoundsError<$int, Bound>;
//     fn try_from(value: Unbounded<UnboundedVal, $int, Bound>) -> Result<Self, Self::Error> {
//         match value.0 {
//             Ok(val) => Ok($type::<MIN, MAX>::from(val)),
//             Err(err) => Err(err),
//         }
//     }
// }

// impl Add<Unbounded<UnboundedVal, $int, Bound>> for Unbounded<UnboundedVal, $int, Bound> {
//     type Output = Unbounded<UnboundedVal, $int, Bound>;
//     fn add(self, other: Unbounded<UnboundedVal, $int, Bound>) -> Self::Output {
//         match (self.0, other.0) {
//             (Ok(self_val), Ok(other_val)) => Ok(self_val + other_val).into(),
//             (Ok(self_val), Err(other_err)) => Err((self_val + other_err.0, other_err.1)).into(),
//             (Err(self_err), Ok(other_val)) => Err((self_err.0 + other_val, self_err.1)).into(),
//             (Err(mut self_err), Err(mut other_err)) => Err((self_err.0 + other_err.0, {
//                 self_err.1.append(&mut other_err.1);
//                 self_err.1
//             }))
//             .into(),
//         }
//     }
// }

// impl<const MIN: Bound, const MAX: Bound> Add<Unbounded<UnboundedVal, $int, Bound>>
//     for $type<MIN, MAX>
// {
//     type Output = Unbounded<UnboundedVal, $int, Bound>;
//     fn add(self, other: Unbounded<UnboundedVal, $int, Bound>) -> Self::Output {
//         self.into_unbounded() + other
//     }
// }


// allow for some operations and comparisons with regular integer types.
derive_numeric_traits!($type, $bound, $int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);



    };
}

mod i8 {
    generate_type!(BoundedI8, i8, i8);
}

mod i16 {
    generate_type!(BoundedI16, i16, i16);
}

mod i32 {
    generate_type!(BoundedI32, i32, i32);
}

mod i64 {
    generate_type!(BoundedI64, i64, i64);
}

mod i128 {
    generate_type!(BoundedI128, i128, i128);
}

mod isize {
    generate_type!(BoundedIsize, isize, isize);
}

mod u8 {
    generate_type!(BoundedU8, u8, u8);
}

mod u16 {
    generate_type!(BoundedU16, u16, u16);
}

mod u32 {
    generate_type!(BoundedU32, u32, u32);
}

mod u64 {
    generate_type!(BoundedU64, u64, u64);
}

mod u128 {
    generate_type!(BoundedU128, u128, u128);
}

mod usize {
    generate_type!(BoundedUsize, usize, usize);
}

#[cfg(test)]
mod tests {
    use super::i64::BoundedI64;
    use assert2::assert;
    use std::convert::TryFrom;

    #[test]
    fn bounded_int_positive() {
        let val0_ok: BoundedI64<0, 10> = 0.into();
        let val1_ok: BoundedI64<0, 10> = 10.into();
        let val2_ok: BoundedI64<30, 100> = 55.into();
        let val3_ok: BoundedI64<5, 5> = 5.into();

        let val0_err: BoundedI64<1, 10> = 0.into();
        let val1_err: BoundedI64<1, 10> = 11.into();
        let val2_err: BoundedI64<102, 121> = 0.into();
        let val3_err: BoundedI64<10, 54> = (-10).into(); // negative value out of bounds, this is a reason why Range would be great as const type, but that isn't possible within stable rust yet.
        let val4_err: BoundedI64<54, 10> = 20.into(); // unexpected order

        assert!(val0_ok.is_ok());
        assert!(val1_ok.is_ok());
        assert!(val2_ok.is_ok());
        assert!(val3_ok.is_ok());

        assert!(val0_err.is_err());
        assert!(val1_err.is_err());
        assert!(val2_err.is_err());
        assert!(val3_err.is_err());
        assert!(val4_err.is_err());
    }

    #[test]
    fn display_int() {
        let ok: BoundedI64<0, 10> = 5.into();
        let err: BoundedI64<0, 10> = 11.into();
        assert_eq!(&format!("{}", ok), "5");
        assert_eq!(
            &format!("{}", err),
            r#"OutOfBoundsError { value: 11, min_allowed: 0, max_allowed: 10 }"#
        );
    }

    #[test]
    fn int_equality_with_literal() {
        let bounded: BoundedI64<0, 10> = 5.into();
        assert!(bounded == 5);
        assert!(5 == bounded);
        assert!(bounded != 6);
        assert!(6 != bounded);
    }

    #[test]
    fn int_equality_with_numerics() {
        let bounded: BoundedI64<0, 10> = 0.into();

        assert!(bounded == 0_u8);
        assert!(bounded == 0_u16);
        assert!(bounded == 0_u32);
        assert!(bounded == 0_u64);
        assert!(bounded == 0_u128);
        assert!(bounded == 0_usize);

        assert!(0_u8 == bounded);
        assert!(0_u16 == bounded);
        assert!(0_u32 == bounded);
        assert!(0_u64 == bounded);
        assert!(0_u128 == bounded);
        assert!(0_usize == bounded);

        assert!(bounded == 0_i8);
        assert!(bounded == 0_i16);
        assert!(bounded == 0_i32);
        assert!(bounded == 0_i64);
        assert!(bounded == 0_i128);
        assert!(bounded == 0_isize);

        assert!(0_i8 == bounded);
        assert!(0_i16 == bounded);
        assert!(0_i32 == bounded);
        assert!(0_i64 == bounded);
        assert!(0_i128 == bounded);
        assert!(0_isize == bounded);
    }

    #[test]
    fn ordering_literal() {
        let bounded_ok: BoundedI64<2, 10> = 5.into();

        assert!(bounded_ok == 5);

        assert!(bounded_ok >= 5);
        assert!(bounded_ok >= 4);
        assert!(bounded_ok >= 1);
        assert!(bounded_ok > 4);
        assert!(bounded_ok > 1);

        assert!(bounded_ok <= 5);
        assert!(bounded_ok <= 6);
        assert!(bounded_ok <= 11);
        assert!(bounded_ok < 6);
        assert!(bounded_ok < 11);

        assert_eq!(bounded_ok <= 4, false);
        assert_eq!(bounded_ok < 5, false);
        assert_eq!(bounded_ok >= 6, false);
        assert_eq!(bounded_ok > 5, false);
    }

    #[test]
    fn ordering_literal_flip() {
        let bounded_ok: BoundedI64<2, 10> = 5.into();

        assert!(5 <= bounded_ok);
        assert!(4 <= bounded_ok);
        assert!(1 <= bounded_ok);
        assert!(4 < bounded_ok);
        assert!(1 < bounded_ok);

        assert!(5 >= bounded_ok);
        assert!(6 >= bounded_ok);
        assert!(11 >= bounded_ok);
        assert!(6 > bounded_ok);
        assert!(11 > bounded_ok);

        assert_eq!(4 >= bounded_ok, false);
        assert_eq!(5 > bounded_ok, false);
        assert_eq!(6 <= bounded_ok, false);
        assert_eq!(5 < bounded_ok, false);
    }

    #[test]
    fn ordering_err_literal() {
        let bounded_err: BoundedI64<2, 10> = 11.into();

        assert_eq!(bounded_err == 11, false);
        assert_eq!(bounded_err > 5, false);
        assert_eq!(bounded_err > 12, false);
        assert_eq!(bounded_err < 5, false);
        assert_eq!(bounded_err < 12, false);
        assert_eq!(bounded_err >= 5, false);
        assert_eq!(bounded_err >= 12, false);
        assert_eq!(bounded_err <= 5, false);
        assert_eq!(bounded_err <= 12, false);
    }

    #[test]
    fn ordering_err_literal_flip() {
        let bounded_err: BoundedI64<2, 10> = 11.into();

        assert_eq!(11 == bounded_err, false);
        assert_eq!(5 < bounded_err, false);
        assert_eq!(12 < bounded_err, false);
        assert_eq!(5 > bounded_err, false);
        assert_eq!(12 > bounded_err, false);
        assert_eq!(5 <= bounded_err, false);
        assert_eq!(12 <= bounded_err, false);
        assert_eq!(5 >= bounded_err, false);
        assert_eq!(12 >= bounded_err, false);
    }

    #[test]
    fn ordering_bounded() {
        let bounded_1: BoundedI64<2, 10> = 4.into();
        let bounded_2: BoundedI64<2, 10> = 6.into();
        let bounded_err_1: BoundedI64<2, 10> = 1.into();
        let bounded_err_2: BoundedI64<2, 10> = 11.into();

        assert!(bounded_1 < bounded_2);
        assert!(bounded_2 > bounded_1);
        assert_eq!(bounded_1 >= bounded_2, false);
        assert_eq!(bounded_2 <= bounded_1, false);

        assert_eq!(bounded_1 <= bounded_err_1, false);
        assert_eq!(bounded_1 >= bounded_err_1, false);
        assert_eq!(bounded_err_1 >= bounded_err_2, false);
        assert_eq!(bounded_err_1 <= bounded_err_2, false);
    }

    #[test]
    fn eq_bounded() {
        let bounded_1: BoundedI64<2, 10> = 4.into();
        let bounded_2: BoundedI64<2, 10> = 6.into();
        let bounded_err_1: BoundedI64<2, 10> = 1.into();
        let bounded_err_2: BoundedI64<2, 10> = 11.into();

        assert!(bounded_1 == bounded_1);
        assert!(bounded_2 != bounded_1);
        assert!(bounded_1 != bounded_2);

        // ::ne has to be a strict inverse of ::eq, so un-equalities with err have to return true
        assert!(bounded_err_1 != bounded_2);
        assert!(bounded_err_1 != bounded_err_2);
        assert!(bounded_err_2 != bounded_err_1);
        assert!(!(bounded_err_1 == bounded_2));
        assert!(!(bounded_err_1 == bounded_err_2));
        assert!(!(bounded_err_2 == bounded_err_1));
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        let parsed = i64::from_str("100").unwrap();
        let parsed_in_bounds = BoundedI64::<0,200>::from_str("100").unwrap();
        let parsed_out_of_bounds = BoundedI64::<0,50>::from_str("100").unwrap();

        assert!(parsed == parsed_in_bounds);
        assert!(parsed_out_of_bounds.is_err());
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn illegal_operations() {
        use trybuild::TestCases;
        let failcheck = TestCases::new();
        failcheck.compile_fail("src/compile_test/must_fail/*.rs");

        // some expected test results depend on whether $int is signed or unsigned
        let signed_only_test = |x: &str| match <i32>::try_from(-1_i32) {
            Ok(_) => failcheck.pass(x),
            Err(_) => failcheck.compile_fail(x),
        };

        signed_only_test("src/compile_test/bounds_1.rs");
        signed_only_test("src/compile_test/bounded_int_negative.rs");
    }

    #[test]
    fn test_sizeof() {
        use super::*;
        use std::mem::size_of;
        assert!(size_of::<Option<i8>>() == size_of::<BoundedI8<0, 10>>());
        assert!(size_of::<Option<i16>>() == size_of::<BoundedI16<0, 10>>());
        assert!(size_of::<Option<i32>>() == size_of::<BoundedI32<0, 10>>());
        assert!(size_of::<Option<i64>>() == size_of::<BoundedI64<0, 10>>());
        assert!(size_of::<Option<i128>>() == size_of::<BoundedI128<0, 10>>());
        assert!(size_of::<Option<isize>>() == size_of::<BoundedIsize<0, 10>>());

        assert!(size_of::<Option<u8>>() == size_of::<BoundedU8<0, 10>>());
        assert!(size_of::<Option<u16>>() == size_of::<BoundedU16<0, 10>>());
        assert!(size_of::<Option<u32>>() == size_of::<BoundedU32<0, 10>>());
        assert!(size_of::<Option<u64>>() == size_of::<BoundedI64<0, 10>>());
        assert!(size_of::<Option<u128>>() == size_of::<BoundedU128<0, 10>>());
        assert!(size_of::<Option<usize>>() == size_of::<BoundedUsize<0, 10>>());
    }
}
