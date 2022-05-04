use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
//Display Text To Prompt User To Guess
    println!("Please Bruv Try And Guess Any Number Between 1 and 10");
    loop{
        println!("Please Input Your Guess Here : ");

// storing values with variables 
        let mut guess  = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading");
        let mut rng = thread_rng();

        println!("Mmmh...You Guessed: {}", guess);
        let secret_number: u32 = rng.gen_range(1..10);
//To parse guess so it matches the secret number data value
    //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let parse_result = guess.trim().parse();
            // Destructure the result
        let guess: u32 = match parse_result {
// If parse succeeded evaluate to the number
            Ok(num) => num,
// If parse failed 
            Err(_) => 0,
        };
//If Else Case To Have A Final Say
        if secret_number == guess {
           println!("Wow Bruv :) You Guessed Correctly!{} {}",secret_number ,guess);
        } 
        else {
            println!("HaHa! You Guessed Wrong :( {} {}",secret_number ,guess);
            match guess.cmp(&secret_number){
                Ordering::Less => println!("So Close But Too Low"),
                Ordering::Greater => println!("So Close But Too High"),
                Ordering::Equal   => {
                    println!("You Are One Badass Sharpshooter");
                    break;
                }

            }
            
        } 
    }
}