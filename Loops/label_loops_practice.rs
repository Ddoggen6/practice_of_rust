use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();

    let n1: i32 = input1.trim().parse().unwrap();
    let n2: i32 = input2.trim().parse().unwrap();

    'outer: for i in 1..=n1 {
        for j in 1..=n2 {
            if i + j > n1 && (i * j) % n2 == 0 {
                println!("({i}, {j})");
                break 'outer;
            }
        }
    }
}
