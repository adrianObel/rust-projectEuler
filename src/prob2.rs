use std::mem;
use std::iter::AdditiveIterator;

fn print_question() {
    println!("Problem #2 ");
    println!("===================================");
    println!("{}{}{}{}\n{}{}", "Each new term in the Fibonacci sequence is "
        , "generated by adding the previous two terms. By starting with 1 "
        , "and 2, the first 10 terms will be:  1, 2, 3, 5, 8, 13, 21, 34, "
        , "55, 89, ..."
        , "* By considering the terms in the Fibonacci sequence whose values "
        , "do not exceed four million, find the sum of the even-valued terms.\n");
}

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
    print_question();

    fibonacchi()
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum()
}
