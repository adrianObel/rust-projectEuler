extern crate num;

use self::num::integer::lcm;

fn print_question() {
    println!("Problem #5");
    println!("===================================");
    print!("What is the smallest positive number that is evenly divisible ");
    println!("by all of the numbers from 1 to 20?.\n");
}

fn find(num: uint) -> uint {
    if num == 20 { return num; }
    lcm(num, find(num + 1))
}

pub fn solve() -> uint {
    print_question();

    find(1)
}
