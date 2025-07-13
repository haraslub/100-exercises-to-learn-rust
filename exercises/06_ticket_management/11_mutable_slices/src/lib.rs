// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

pub fn squared(slice: &mut [i32]) -> &mut [i32] {
    for item in slice.iter_mut() {
        // "item" is a reference (i.e. an address) where the actual value is stored
        // i.e, the "item" is not the number itself 
        // so, to work with the actual values, we need to dereference it
        *item = *item * *item;
        // ↑       ↑       ↑
        // │       │       └── Get the value to multiply
        // │       └────────── Get the value to multiply  
        // └────────────────── Put the result back at this location

        // item = item * item;  // ❌ Error: can't multiply references!
    }
    slice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
