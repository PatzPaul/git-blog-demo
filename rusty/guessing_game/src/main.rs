use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Please Bruv Have a Guess");
    println!("please input your guess: ");

// storing values with variables 
    let mut guess  = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading");
    let mut rng = thread_rng();

    println!("you guessed: {}", guess);
    let secret_number: u32 = rng.gen_range(1..100);
       
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
            
    } 
                //println!("The Secret number is : {}", secret_number);
    

}
