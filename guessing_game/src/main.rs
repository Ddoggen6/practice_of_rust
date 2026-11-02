use std::io;

fn main() {
    println!("Guess the number!");

    printfln!("Please input your guess.");

    let mut guess: Sting::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {guess}");
}
