

// #################### Guess Game ####################

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");

        let mut guess= String::new(); // by default variable is immutable then need to add mut

        io::stdin()
            .read_line(&mut guess) // use & to reference variable and add mut to make it mutable
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win !!!");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
}