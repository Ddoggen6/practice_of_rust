use std::io;

fn main() {
    println!("Enter a numeric limit:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let limit: i32 = input.trim().parse().unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        println!("Count: {counter}");

        if counter >= limit {
            break;
        }
    }
}
