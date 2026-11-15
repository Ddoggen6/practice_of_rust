use std::io;

fn main() {
    println!("Calculator App");

    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let num1: f64 = input1.trim().parse().unwrap();
    let num2: f64 = input2.trim().parse().unwrap();

    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;

    println!("Sum: {:.2}", sum);
    println!("Difference: {:.2}", difference);
    println!("Product: {:.2}", product);
    println!("Quotient: {:.2}", quotient);
}
