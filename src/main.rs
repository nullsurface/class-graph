use std::io;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Node {
    radius: i32,
    color: Color,
}

fn main() {
    let mut read_in = String::new();

    io::stdin()
    .read_line(&mut read_in)
    .expect("You fucked up.");

    println!("{read_in}");
}
