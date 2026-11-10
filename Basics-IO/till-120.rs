use std::io;

fn main() {
    println!("Enter your age:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let age: i32 = input.trim().parse().unwrap();

    let result: i32 = 120 - age;

    println!("{result} years till 120");
}
