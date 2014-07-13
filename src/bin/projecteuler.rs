use std::os;

mod prob1;
mod prob2;
mod prob4;
mod prob5;
mod prob6;
mod prob7;
mod prob8;
mod prob10;

fn main() {
    let arguments = os::args();
    let arguments = arguments.iter().map(|x| x.as_slice())
        .collect::<Vec<&str>>();

    println!("Answer is: {}", match arguments.as_slice() {
        [_, "--prob", "001"] => prob1::solve(),
        [_, "--prob", "002"] => prob2::solve(),
        [_, "--prob", "004"] => prob4::solve(),
        [_, "--prob", "005"] => prob5::solve(),
        [_, "--prob", "006"] => prob6::solve(),
        [_, "--prob", "007"] => prob7::solve(),
        [_, "--prob", "008"] => prob8::solve(),
        [_, "--prob", "010"] => prob10::solve(),
        _ => fail!("Problem not resolved")
    });

}
