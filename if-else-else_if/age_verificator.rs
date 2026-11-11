use std::io;

fn main() {
    println!("Enter your age:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = input.trim().parse().unwrap();

    if age > 17 {
        println!("You are an adult buddy")
    } else {
        println!("you are still a child")
    };
}
