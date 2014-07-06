use std::os;

mod prob1;
mod prob2;
mod prob5;
mod prob6;

fn main() {
    let arguments = os::args();
    let arguments = arguments.iter().map(|x| x.as_slice())
        .collect::<Vec<&str>>();

    println!("Answer is: {}", match arguments.as_slice() {
        [_, "--prob", "001"] => prob1::solve(),
        [_, "--prob", "002"] => prob2::solve(),
        [_, "--prob", "005"] => prob5::solve(),
        [_, "--prob", "006"] => prob6::solve(),
        _ => fail!("Problem not resolved")
    });

}
