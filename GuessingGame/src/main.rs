use std::io;
use rand::RngExt;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number ! ");
    
    let secret_number = rand::rng().random_range(1..101);
    println!("Generate a secrate number ");
    loop{
    println!("Please input your guess");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");// refference passed
    
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) =>{ 
            println!("Invalid input. Please enter a number.");
            continue;
        },
    } ;//shadowing 
    println!("You guessed the number {}", guess);
    
    match guess.cmp(&secret_number){
        Ordering::Less => {println!("Too small!")},
        Ordering::Greater => {println!("Too big!")},
        Ordering::Equal => {println!("You win!"); break;},
    }
    }
    
}
