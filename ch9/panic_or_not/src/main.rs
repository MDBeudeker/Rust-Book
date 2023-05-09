/* 

// Fixing the guessing game code with more error handling

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..100);


    loop{
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input is not a number.");
                continue;
            }
        };
        
        if guess < 1 || guess > 100{
            println!("The guess can only be between 1 and 100.");
            continue;
        }

        println!("You guessed {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}
*/


// using a struct, this does not compile and at this point i dont really care

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..100);


    loop{
        println!("Please input your guess:");

        pub struct Guess{
            value: i32,
        }

        impl Guess{
            pub fn new(value:i32) -> Guess{
                if value < 1 || value > 100{
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }
                Guess { value }
            }
            pub fn value(&self) -> i32{
                self.value
            }
        }



        let mut guess = Guess::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        // let guess: i32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Input is not a number.");
        //         continue;
        //     }
        // };
        
        if guess < 1 || guess > 100{
            println!("The guess can only be between 1 and 100.");
            continue;
        }

        println!("You guessed {:?}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}
