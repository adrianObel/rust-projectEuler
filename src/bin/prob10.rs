extern crate math;

use std::iter::AdditiveIterator;
use self::math::prime::Prime;

fn print_question() {
    println!("Problem #10");
    println!("===================================");
    println!("Find the sum of all the primes below two million.\n");
}

pub fn solve() -> uint {
    print_question();

    let sequence = Prime::new();

    sequence
        .take_while(|&p| p < 2_000_000)
        .sum()

}
