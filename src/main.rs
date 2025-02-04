extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret);

    loop {
        println!("Insert number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            } 
        }
    
        println!("Your guess: {}", guess);
    }
}
