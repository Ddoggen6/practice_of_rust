use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for i in 1..=n {
        if n == 3 {
            break;
        }
        println!("{i}");
    }
}
