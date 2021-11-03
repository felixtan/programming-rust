// FromStr is a trait. it's brought into scope here
// types that implement FromStr have from_str method
use std::str::FromStr;

// this module provides functions and types for interacting with execution environment
use std::env;

fn main() {
    let mut numbers = Vec::new();

    // first value produced by env::args() is the name of the program
    for arg in env::args().skip(1) {
        // u64::from_str returns a Result value indicating whether the parse succeeded or failed
        // expect is a method of Result that prints a message on Err or returns value on Ok
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // m is a reference to an element in numbers
    for m in &numbers[1..] {
        // *m dereferences m and yields the value it referenced
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                3 * 11);
}