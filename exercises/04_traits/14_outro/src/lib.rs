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

use std::convert::From;
use std::ops::Add;

// Define a custom type that wraps a u16 value and handles arithmetic saturation
// (meaning it won't overflow, but instead stay at the maximum value)
#[derive(Copy, Clone, Debug, PartialEq)]  // Automatically implement these traits for our type
pub struct SaturatingU16 {
    value: u16,  // Internal u16 value that we're wrapping
}

// Convert from a regular u16 to our SaturatingU16
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        SaturatingU16 { value: value }  // Simply wrap the value in our struct
    }
}

// Convert from a reference to u16 to our SaturatingU16
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        SaturatingU16 { value: (*value) }  // Dereference the value and wrap it
    }
} 

// Convert from u8 to our SaturatingU16
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        SaturatingU16 { value: value.into() }  // Convert u8 to u16 automatically and wrap it
    }
}

// Convert from a reference to u8 to our SaturatingU16
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> SaturatingU16 {
        SaturatingU16 { value: (*value).into() }  // Dereference and convert u8 to u16
    }
}

// Convert from a reference to SaturatingU16 to a new SaturatingU16
impl From<&SaturatingU16> for SaturatingU16 {
    fn from(wrapped: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 { value: (wrapped.value) }  // Create a new instance with the same value;
        // no need an explicit derefence as operator '.' does it automatically
    }
}

// Add a u16 to our SaturatingU16
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;  // The result type of addition

    fn add(self, other: u16) -> Self {
        Self {
            value: self.value.saturating_add(other)  // Use saturating_add to prevent overflow
        }
    }
}

// Add a reference to u16 to our SaturatingU16
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> Self {
        Self {
            value: self.value.saturating_add(*other)  // Dereference and add
        }
    }
}

// Add a u8 to our SaturatingU16
impl Add<u8> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u8) -> Self {
        Self {
            value: self.value.saturating_add(other as u16)  // Convert u8 to u16 and add
        }
    }
}

// Add a reference to u8 to our SaturatingU16
impl Add<&u8> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u8) -> Self {
        Self {
            value: self.value.saturating_add(*other as u16)  // Dereference, convert to u16, and add
        }
    }
}

// Add a reference to SaturatingU16 to our SaturatingU16
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> Self {
        Self {
            value: self.value.saturating_add(other.value)  // Add the internal values
            // no need an explicit derefence as operator '.' does it automatically
        }
    }
}

// Add two SaturatingU16 values together
impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: Self) -> SaturatingU16 {
        Self {
            value: self.value.saturating_add(other.value)  // Add the internal values
        }
    }
}

// Compare SaturatingU16 with u16 for equality
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other  // Compare the internal value with the u16
    }
}

// Compare u16 with SaturatingU16 for equality (reverse of above)
impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.value  // Compare the u16 with the internal value
    }
}

