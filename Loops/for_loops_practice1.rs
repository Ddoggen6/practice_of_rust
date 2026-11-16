use std::io;

fn main() {
    println!("Enter the distance or group of numbers you want to add between them. Example: add all the numbers from 1 to 100.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    let mut sum_of_numbers = 0;
    for i in 1..=n {
        sum_of_numbers += i;
    }

    println!("{}", sum_of_numbers);
}
