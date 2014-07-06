extern crate num;

use self::num::integer::lcm;

fn find(num: uint) -> uint {
    if num == 20 { return num; }
    lcm(num, find(num + 1))
}
pub fn solve() -> String {
    find(1).to_str()
}
