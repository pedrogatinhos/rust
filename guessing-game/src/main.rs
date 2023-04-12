use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");
    println!("input number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");

    println!("guess!");
    println!("your guess is {guess}");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
    
    
}
