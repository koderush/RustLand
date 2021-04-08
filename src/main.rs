use std::io;

mod feature1;

fn main() {
    feature1::calculate_fab(35000000);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}({})", guess.trim(), guess.trim().len());
}
