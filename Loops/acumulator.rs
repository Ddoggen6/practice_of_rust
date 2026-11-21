use std::io;

fn main() {
    println!("Enter the quantity of numbers that will be acumulated:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: i32 = input.trim().parse().unwrap();
    let mut sum: i32 = 0;

    println!("Enter the numbers that will be sumed:");

    for _i in 0..count {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num: i32 = input.trim().parse().unwrap();

        sum = sum + num;
    }
    println!("The result: ");
    println!("{sum}")
}
