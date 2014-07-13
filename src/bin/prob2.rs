extern crate math;

use std::iter::AdditiveIterator;
use self::math::fibonacci::Fibonacci;

fn print_question() {
    println!("Problem #2");
    println!("===================================");
    print!("By considering the terms in the Fibonacci sequence whose values ");
    println!("do not exceed four million, find the sum of the even-valued terms.\n");
}

pub fn solve() -> uint {
    print_question();

    let series = Fibonacci::new();

    series
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum()

}
