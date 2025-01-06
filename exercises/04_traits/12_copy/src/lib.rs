// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::convert::From<u32> for WrappingU32 {
    fn from(value: u32) -> WrappingU32 {
        WrappingU32 { value }
    }
}

impl std::ops::Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
