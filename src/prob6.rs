use std::iter::AdditiveIterator;

pub fn solve() -> uint {
    let limit = 101;
    let sum_of_square = range(1u, limit)
        .map(|x| x * x)
        .sum();

    let square_of_sum = range(1u, limit).sum();
    let square_of_sum = square_of_sum * square_of_sum;

    square_of_sum - sum_of_square
}
