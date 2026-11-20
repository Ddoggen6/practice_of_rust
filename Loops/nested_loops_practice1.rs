use std::io;

fn main() {
    let mut width_input = String::new();
    let mut height_input = String::new();
    io::stdin().read_line(&mut width_input).unwrap();
    io::stdin().read_line(&mut height_input).unwrap();
    let width: i32 = width_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();
    // Write your code below

    for x in 0..width {
        for y in 0..height {
            println!("{width} {height}");
        }
    }
}
