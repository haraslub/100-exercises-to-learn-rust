// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    let first_half: Vec<i32> = (&v.clone()[..len/2]).to_vec();
    let second_half: Vec<i32> = (&v.clone()[len/2..]).to_vec();

    println!("First half: {:#?}", first_half);
    println!("Second half: {:#?}", second_half);
    
    // let first_handle = thread::spawn(move || {
    //     let mut total: i32 = 0;
    //     for i in first_half.iter() {
    //         total += i;
    //     };
    //     total
    // });

    // let second_handle = thread::spawn(move || {
    //     let mut total: i32 = 0;
    //     for i in second_half.iter() {
    //         total += i;
    //     };
    //     total
    // });

    // optimized
    let sum_closure = |to_sum: Vec<i32>| {
        let mut total: i32 = 0;
        for i in to_sum.iter() {
            total += i;
        };
        total
    };

    let first_handle = thread::spawn(move || sum_closure(first_half));
    let second_handle = thread::spawn(move || sum_closure(second_half));

    // return the sum of both
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
