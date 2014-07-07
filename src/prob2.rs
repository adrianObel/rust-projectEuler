use std::mem;
use std::iter::AdditiveIterator;

struct Fibonacchi {
    curr: uint,
    next: uint
}

impl Iterator<uint> for Fibonacchi {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = mem::replace(&mut self.next, new_next);

        Some(mem::replace(&mut self.curr, new_curr))
    }
}

fn fibonacchi() -> Fibonacchi {
    Fibonacchi { curr: 1, next: 1 }
}

pub fn solve() -> uint {
    fibonacchi()
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum()
}
