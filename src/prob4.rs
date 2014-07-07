fn print_question() {
    println!("Problem #4");
    println!("===================================");
    println!("{}{}{}\n{}", "A palindromic number reads the same both ways. ",
        "The largest palindrome made from the product of two 2-digit numbers is",
        " 9009 = 91 × 99.",
        "* Find the largest palindrome made from the product of two 3-digit numbers.\n")
}
fn is_palindrome(num: uint) -> bool {
    let mut n = num.clone();
    let mut rev = 0;

    while n != 0 {
        rev *= 10;
        rev += n % 10;
        n /= 10;
    }

    rev == num
}

pub fn solve() -> uint {
    print_question();

    let mut largest = 0;
    for i in range(100, 999) {
        for j in range(100, 999) {

            let product = i * j;

            if is_palindrome(product) && product > largest{
                largest = product
            }
        }
    }

    largest
}
