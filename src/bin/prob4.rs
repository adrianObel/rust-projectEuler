fn print_question() {
    println!("Problem #4");
    println!("===================================");
    print!("Find the largest palindrome made from ");
    println!("the product of two 3-digit numbers.\n");
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

    let base = 100u;
    let limit = 1000;

    range(base, limit)
        .filter_map(|i| {
            range(base, limit)
                .map(|j| i * j)
                .filter(|&p| is_palindrome(p))
                .max()
        })
        .max()
        .unwrap()
}
