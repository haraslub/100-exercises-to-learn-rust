// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref FIB_NUMBERS: Mutex<Vec<u32>> = Mutex::new(vec![0, 1, 1, 2, 3, 5, 8, 13, 21]);
}

pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let mut fib_numbers = FIB_NUMBERS.lock().unwrap();
    // get takes input usiye
    if let Some(fib) = fib_numbers.get(n as usize) {
        // get returns reference, so we need to dereference
        return *fib
    }

    while fib_numbers.len() <= n as usize {
        let next = fib_numbers[fib_numbers.len() - 1] + fib_numbers[fib_numbers.len() - 2];
        fib_numbers.push(next);
    }

    fib_numbers[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
