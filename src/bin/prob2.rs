use std::mem::replace;
use std::iter::AdditiveIterator;

fn print_question() {
    println!("Problem #2");
    println!("===================================");
    print!("By considering the terms in the Fibonacci sequence whose values ");
    println!("do not exceed four million, find the sum of the even-valued terms.\n");
}

struct Fibonacchi {
    curr: uint,
    next: uint
}

impl Iterator<uint> for Fibonacchi {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacchi() -> Fibonacchi {
    Fibonacchi { curr: 1, next: 1 }
}

pub fn solve() -> uint {
    print_question();

    fibonacchi()
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum()

}
