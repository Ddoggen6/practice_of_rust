use std::io;

fn main() {
    let mut n1_input = String::new();
    let mut n2_input = String::new();
    let mut op_input = String::new();

    io::stdin().read_line(&mut n1_input).unwrap();
    io::stdin().read_line(&mut n2_input).unwrap();
    io::stdin().read_line(&mut op_input).unwrap();

    let n1: f64 = n1_input.trim().parse().unwrap();
    let n2: f64 = n2_input.trim().parse().unwrap();
    let op = op_input.trim();

    let result: f64 = if op == "+" {
        n1 + n2
    } else if op == "-" {
        n1 - n2
    } else if op == "/" {
        n1 / n2
    } else {
        n1 * n2
    };

    println!("{}", result);
}
