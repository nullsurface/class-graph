use std::io;

fn main() {
    let mut read_in = String::new();

    io::stdin()
    .read_line(&mut read_in)
    .expect("You fucked up.");

    println!("{read_in}");
}
