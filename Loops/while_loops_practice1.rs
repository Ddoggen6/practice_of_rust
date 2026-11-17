use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut number: f64 = input.trim().parse().unwrap();

    while number >= 10.00 {
        number /= 2.00;
    }
    println!("{number}");
}
