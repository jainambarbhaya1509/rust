use std::io;
use std::cmp::Ordering;
use rand::Rng; // rand -> package | Rng -> random number generator

fn main() {

    let _secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", _secret_number);
    loop {
        println!("Guess the number: ");
    
        let mut guess = String::new(); // mut is mutable default is immutable

        // read_line is a method of the io::stdin() struct
        // read_line takes a mutable reference to a string
        // expect is a method that takes a string and returns a Result
        // if the result is an error, it will panic with the message
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        println!("your guess {}", guess);

        match guess.cmp(&_secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            },
        }
    }

    let x = 10;
    let y = 20;
    // x=1; wont work because x is immutable
    println!("x: {}, y+2: {}",x, y+2);
}


// Result is an enum with two variants: Ok and Err
// Ok is used for success and Err is used for error
// Result is used to handle errors in Rust
// Result is a type that can be used to return a value or an error