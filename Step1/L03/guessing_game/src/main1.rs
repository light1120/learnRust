use rand::Rng;
use std::io;

fn println(str: &str) {
    println!("{str}")
}

fn main() {
    println("Hello, Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_world = format!("The secret number is {secret_number}");
    println(&secret_world);

    println("please type a word");

    let mut world = String::new();

    io::stdin().read_line(&mut world).expect("somthing wrong");

    println!("Your world is {world}");
}
