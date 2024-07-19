use std::io;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("{}", "Guessing Game!".yellow());

    let fruits = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
    ];

    for (index, value) in fruits.iter().enumerate() {
        println!("{}", format!("{}: {}", index, value).purple());
    }

    let secret_fruit = fruits[rand::thread_rng().gen_range(0..fruits.len())];

    loop {
        println!("{}", "Please enter your guess fruit.".yellow());

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess = guess.to_lowercase();

        let guess = guess.trim();

        if guess == secret_fruit {
            println!("{}", "Correct Guess! You are winner!".green().bold());
            break;
        } else {
            println!("{}", "Your guess is wrong!".red().bold());
        }
    }
}
