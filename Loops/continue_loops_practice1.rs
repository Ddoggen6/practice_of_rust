use std::io;

fn main() {
    println!("Only even numbers can be printed:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for i in 0..=n {
        if i % 2 != 0 {
            continue;
        }

        println!("{i}");
    }
}
