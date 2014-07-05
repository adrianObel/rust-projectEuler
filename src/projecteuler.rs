use std::os;

fn main() {
    let arguments = os::args();
    let arguments = arguments.iter().map(|x| x.as_slice())
        .collect::<Vec<&str>>();

    match arguments.as_slice() {
        [_, "--prob", "001"] => println!("Solve problem 001"),
        _ => ()
    }

}
