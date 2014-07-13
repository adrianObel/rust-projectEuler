use std::mem::replace;
use std::iter::range_step;

pub struct Prime {
    curr: uint,
    next: uint
}

impl Prime {
    pub fn new() -> Prime {
        Prime { curr: 2, next: 3}
    }
}

impl Iterator<uint> for Prime {
    fn next(&mut self) -> Option<uint> {

        let new_next = calculate_next_prime(self.next);
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }}

fn calculate_next_prime(start_value: uint) -> uint {
    let mut start_point = start_value + 1;

    loop {
        if is_prime(start_point) {
            return start_point;
        } else {
            start_point += 1;
        }
    }
}

pub fn is_prime(num: uint) -> bool {
    let root = (num as f64).sqrt().ceil() as uint;

    if num == 2 || num == 3 { return true; }
    if num % 2 == 0 { return false; }

    for i in range_step(3u, root + 1, 2) {
        if num % i == 0 { return false; }
    }

    true
}

// Tests

#[test]
fn test_is_prime() {
    assert!(is_prime(3) == true);
    assert!(is_prime(4) == false);
    assert!(is_prime(9) == false);
}
