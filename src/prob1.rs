use std::iter::AdditiveIterator;

pub fn solve() -> String {
    range(0u, 1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
        .to_str()
}
