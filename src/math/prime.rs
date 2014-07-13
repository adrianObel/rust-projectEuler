use std::mem;

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
        let new_curr = mem::replace(&mut self.next, new_next);

        Some(mem::replace(&mut self.curr, new_curr))
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

}

// Tests

#[test]
fn test_is_prime() {
    assert!(is_prime(3) == true);
    assert!(is_prime(4) == true);
    assert!(is_prime(9) == false);
}
