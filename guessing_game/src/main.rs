use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut x = 5;
    x = 6;
    loop {
        println!("Please input your guess.");
        
        //mutable variable: variable that allowed to be updated
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            //expect: if io::Result is Err, expect will crash the program and display the message
            .expect("Failed to read line");

        //changing guess from string to unsigned 32-bit integer
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        //comparing value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
