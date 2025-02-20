// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug,Clone, Copy, PartialEq)]
pub struct SaturatingU16{
    value: u16
}

impl From<u16> for SaturatingU16 {
    /// Creates a new `SaturatingU16` from a given `u16` value, by directly
    /// copying the value.
    fn from(value: u16) -> Self {
        Self { value: value }
    }
}

impl From<u8> for SaturatingU16{
    /// Creates a new `SaturatingU16` from a given `u8` value by casting it to a
    /// `u16` and directly copying the value.
    fn from(value: u8) -> Self {
        Self { value: value.into() }
    }
}

impl From<&u16> for SaturatingU16 {
    /// Creates a new `SaturatingU16` from a given reference to a `u16` value by
    /// directly copying the value.
    fn from(value: &u16) -> Self {
        Self {
            value: (*value).into()
        }
    }
}

impl From<&u8> for SaturatingU16 {
/// Creates a new `SaturatingU16` from a given reference to a `u8` value by
/// casting the `u8` to a `u16` and directly copying the value.
    fn from(value: &u8) -> Self {
        Self {
            value: (*value).into()
        }
    }
}

use std::ops::Add;

impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    /// Saturating addition of two `SaturatingU16` values.
    ///
    /// This method will add the two values, and if the result is greater than
    /// `u16::MAX`, it will return `u16::MAX`. If the result is less than
    /// `u16::MIN`, it will return `u16::MIN`.
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value)
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    /// Saturating addition of a `SaturatingU16` with a reference to another `SaturatingU16`.
    ///
    /// This method will add the two values, and if the result is greater than
    /// `u16::MAX`, it will return `u16::MAX`. If the result is less than
    /// `u16::MIN`, it will return `u16::MIN`.
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add((*rhs).value)
        }
    }
}

impl Add<u16> for SaturatingU16{
    type Output = SaturatingU16;
    /// Saturating addition of a `SaturatingU16` with a `u16`.
    ///
    /// This method will add the two values, and if the result is greater than
    /// `u16::MAX`, it will return `u16::MAX`. If the result is less than
    /// `u16::MIN`, it will return `u16::MIN`.
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.into())
        }
    }
}

impl Add<&u16> for SaturatingU16{
    type Output = SaturatingU16;
    /// Saturating addition of a `SaturatingU16` with a reference to a `u16`.
    ///
    /// This method will add the two values, and if the result is greater than
    /// `u16::MAX`, it will return `u16::MAX`. If the result is less than
    /// `u16::MIN`, it will return `u16::MIN`.
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add((*rhs).into())
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    /// Compares the value of this `SaturatingU16` with the given `u16`.
    ///
    /// This method will return `true` if the values are equal, and `false` otherwise.
    /// 
    /// The PartialEq<u16> implementation is important because it allows you to compare 
    /// a SaturatingU16 value with a u16 value using the == operator.
    /// Without this implementation, you wouldn't be able to compare a SaturatingU16 value 
    /// with a u16 value directly, because Rust wouldn't know how to compare them.
    /// By implementing PartialEq<u16>, you're telling Rust that it's okay to compare
    /// a SaturatingU16 value with a u16 value, and that the comparison should be done
    /// drtby comparing the underlying value field of the SaturatingU16 with the u16 value.
    /// This is useful because it allows you to write code like my_saturating_u16 == 10u16, 
    /// which is more convenient and readable than having to write my_saturating_u16.value == 10u16.
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

