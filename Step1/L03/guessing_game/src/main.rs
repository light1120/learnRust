use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, Guessing Game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("secret number is {}", secret_number);

    loop {
        println!("please type your input.");

        let mut input_number = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("failed read stdin");

        let input_number: u32 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        println!("Your input number is : {}", input_number);

        match input_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("congraduation, u win");
                break;
            }
            Ordering::Less => println!("your input is lesser"),
            Ordering::Greater => println!("your input is greaters"),
        };
    }
}
