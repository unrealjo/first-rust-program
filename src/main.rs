use std::io;
fn main() {
    println!("How old are you ?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading line");
    let age: u32 = input.trim().parse().expect("Failed to parse the input");
    println!("Year of birth: {}", 2023 - age)
}
