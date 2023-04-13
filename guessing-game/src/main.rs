use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");
    println!("input number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    

    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to readline");
        let input: u32 = guess.trim().parse().expect("Please type a number!");
        println!("input guess");

        match input.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("congrats!!");
                break;
            }
        }
        guess.clear();
    };
    
}
