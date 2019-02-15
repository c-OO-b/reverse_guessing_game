use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Reverse Guessing Game");
    println!("Type a number for the machine to guess: ");
    
    let mut rng = rand::thread_rng();
    let mut highest_guess = 100;
    let mut lowest_guess = 1;
    let mut prev_guess = 0;
    let mut secret_number = String::new();

    io::stdin().read_line(&mut secret_number)
        .expect("Getting number didnt work");

    let secret_number: u32 = secret_number.trim().parse()
        .expect("Converting number to int faild");

    loop {
        let guess = rng.gen_range(lowest_guess, highest_guess);
        if prev_guess == guess { continue; } // only half fix
        
        println!("Guessing {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Machine guessed the right number {}", guess);
                break;
            }
            Ordering::Greater => {
                println!("{} is greater then", guess);
                highest_guess = guess;
                prev_guess = guess;
            } 
            Ordering::Less => {
                println!("{} is lower then", guess);
                lowest_guess = guess;
                prev_guess = guess;
          }
        }
    }
}