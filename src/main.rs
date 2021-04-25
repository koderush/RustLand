use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod database;
mod feature1;

fn main() {
    // feature1::calculate_fab(35000000);

    // let records = database::get_edrsystem_records();

    // println!("Found {} edr.edrsystem records", records.unwrap().len());

    let mut rng = rand::thread_rng();

    let secret_number: u32 = rng.gen_range(0..10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong format, try again.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
