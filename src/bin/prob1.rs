use std::iter::AdditiveIterator;

fn print_question() {
    println!("Problem #1");
    println!("===================================");
    println!("Find the sum of all the multiples of 3 or 5 below 1000.\n");
}

pub fn solve() -> uint {
    print_question();

    range(0u, 1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}
