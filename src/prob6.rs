use std::iter::AdditiveIterator;
use std::num::pow;

pub fn solve() -> uint {
    let limit = 101;
    let sum_of_square = range(1u, limit)
        .map(|x| x * x)
        .sum();

    let square_of_sum = pow(range(1u, limit).sum(), 2);

    square_of_sum - sum_of_square
}
