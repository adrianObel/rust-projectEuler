extern crate math;

use self::math::prime::Prime;

fn print_question() {
    println!("Problem #7");
    println!("===================================");
    println!("What is the 10 001st prime number?\n");
}

pub fn solve() -> uint {
    print_question();

    let sequence = Prime::new();

    sequence
        .take(10_001)
        .max()
        .unwrap()

}
