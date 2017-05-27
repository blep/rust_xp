extern crate rand; //use external library named rand. Implicit use rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number beteen 1 and 100 included");
    let secret_number = rand::thread_rng().gen_range(1, 101);
	// println!("The secret number is: {}", secret_number);
	let mut attempt = 1;
	
	loop {
	    println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		let guess: i32 = match guess.trim().parse() { // parse infer signess from guess declared type
            Ok(num) => num,
            Err(_) => continue,
        };
			
		println!("You guessed: {}", guess);
		
		match guess.cmp(&secret_number) {
			Ordering::Less    => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal   => {
				println!("You win in {} attempts!", attempt);
				break;
			}
		}
		attempt += 1;
	}


}
