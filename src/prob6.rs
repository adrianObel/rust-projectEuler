use std::iter::AdditiveIterator;
use std::num::pow;

fn print_question() {
    println!("Problem #6");
    println!("===================================");
    print!("Find the difference between the sum of the squares of the first ");
    println!("one hundred natural numbers and the square of the sum.\n");
}

pub fn solve() -> uint {
    print_question();

    let limit = 101;
    let sum_of_square = range(1u, limit)
        .map(|x| x * x)
        .sum();

    let square_of_sum = pow(range(1u, limit).sum(), 2);

    square_of_sum - sum_of_square
}
