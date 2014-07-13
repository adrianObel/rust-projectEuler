use std::mem::replace;

pub struct Fibonacci {
    curr: uint,
    next: uint
}

impl Iterator<uint> for Fibonacci {
    fn next(&mut self) -> Option<uint> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }
}
