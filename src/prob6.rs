use std::iter::AdditiveIterator;
use std::num::pow;

fn print_question() {
    println!("Problem #6");
    println!("===================================");
    println!("{}{}\n\t{}\n{}\n\t{}\n{}{}\n{}\t{}", "The sum of the squares ",
        "of the first ten natural numbers is: ",
        "1^2 + 2^2 + ... + 10^2 = 385",
        "The square of the sum of the first ten natural numbers is: ",
        "(1 + 2 + ... 10) ^2 = 55^2 = 3025",
        "Hence the difference between the sum of the squares of the first ten ",
        "natural numbers and the square of the sum is 3025 − 385 = 2640.",
        "* Find the difference between the sum of the squares of the first ",
        "one hundred natural numbers and the square of the sum.\n")
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
