
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Welcome to the guessing game!");
    println!("             ");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("the secret number is: {}", secret_number);
    
    
    
    
    loop {

        println!("Please enter your guess:");
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let guess: u32 = input.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;

                }
            }

    }

    println!("Thanks for playing!");

}
