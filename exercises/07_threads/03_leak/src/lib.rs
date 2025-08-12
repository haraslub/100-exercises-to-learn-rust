// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v_leaked: &'static mut [i32] = Vec::leak(v);
    let half = v_leaked.len()/2;
    let v_leaked_first = &v_leaked[..half];
    let v_leaked_second = &v_leaked[half..];

    let sum_closure = |to_sum: &'static [i32]| {
        to_sum.iter().sum::<i32>()
    };

    let first_handle = thread::spawn(move || {sum_closure(v_leaked_first)});
    let second_handle = thread::spawn(move || {sum_closure(v_leaked_second)});

    first_handle.join().unwrap() + second_handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
