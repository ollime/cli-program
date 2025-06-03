use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // redefine as int to prevent mismatched types error
    let guess: u32 = guess.trim().parse().expect("Type a number");

        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win!");
                break;
            },
        }
    }
}
