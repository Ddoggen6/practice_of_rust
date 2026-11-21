use std::io;

fn main() {
    println!("Enter the height and the width:");
    let mut width_input = String::new();
    let mut height_input = String::new();
    io::stdin().read_line(&mut width_input).unwrap();
    io::stdin().read_line(&mut height_input).unwrap();
    let width: i32 = width_input.trim().parse().unwrap();
    let height: i32 = height_input.trim().parse().unwrap();

    for _x in 0..height {
        for _y in 0..width {
            print!("*")
        }
        println!("");
    }
}
