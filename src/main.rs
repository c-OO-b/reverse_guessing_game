use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Reverse Guessing Game");
    println!("Type a number for the machine to guess: ");
    
    let mut rng = rand::thread_rng();
    let mut highest_guess = 100;
    let mut lowest_guess = 1;
    let mut secret_number = String::new();

    let mut guesses = vec![];

    io::stdin().read_line(&mut secret_number)
        .expect("Getting number didnt work");

    let secret_number: u32 = secret_number.trim().parse()
        .expect("Converting number to int failed");

    'outer: loop {
        let guess = rng.gen_range(lowest_guess, highest_guess);
        let mut iter = guesses.clone().into_iter();
        let result: Vec<u32> = iter.by_ref().collect();

        'inner: for i in &result { // Let's make sure we are not guessing the same number twice.
            if *i == guess {
                continue 'outer;
            } 
        }

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("[{}] Machine has guessed the correct number", guess);
                break 'outer;
            }
            Ordering::Greater => {
                highest_guess = guess;
                guesses.push(guess);
                println!("[{}] is HIGHER then the secret number", guess);
            }
            Ordering::Less => {
                lowest_guess = guess;
                guesses.push(guess);
                println!("[{}] is LOWER then the secret number", guess);
            }
        }
    }
}
   