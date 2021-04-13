use std::io;

mod feature1;
mod database;

fn main() {
    feature1::calculate_fab(35000000);

    let records = database::get_edrsystem_records();

    println!("Found {} edr.edrsystem records", records.unwrap().len());

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}({})", guess.trim(), guess.trim().len());
}
