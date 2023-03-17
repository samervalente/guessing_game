use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut username = String::new();
    let mut age = String::new();
    let mut guess = String::new();

    println!("Please input your user name!");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read user name input");

    println!("Hello {username}. How old are you?");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read age input");

    println!("Nice! Now input your guess. Good Luck!");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess input");

    let guess: u32 = guess.trim().parse().expect("Please, type a number!");

    println!(
        "Result: \n username:{} age:{age} guessed: {}, secret_number: {secret_number}",
        username, guess
    );

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!!"),
    }
}
