use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    let greetings = String::from("Hello, players!");
    println!("{}", greetings);

    let mut pleayer_one = String::new();
    let mut player_two = String::new();

    println!("wELCOME TO GUESSING GAME TWO PLAYERS MOOD");

    loop {
        println!("Please enter your name Player One:");
        io::stdin()
            .read_line(&mut pleayer_one)
            .expect("You need to enter One ");

        println!("Please enter your name Player Two:");

        io::stdin()
            .read_line(&mut player_two)
            .expect("You need to enter Two ");

        loop {
            println!("Player one's turn to guess:");
            let mut guess_player_one = String::new();
            io::stdin()
                .read_line(&mut guess_player_one)
                .expect("Player one needs the");

            let guess_player_one = match guess_player_one.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Player one Please enter a valid number.");
                    continue;
                }
            };

            match guess_player_one.cmp(&random_number) {
                Ordering::Less => println!("Too small!  \n"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("Player One wins!");
                    break;
                }
            }
            println!("Player two's turn to guess:");

            let mut guess_player_two = String::new();

            io::stdin()
                .read_line(&mut guess_player_two)
                .expect("Player two must guess");

            let guess_player_two = match guess_player_two.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Player two Please enter a valid number.");
                    continue;
                }
            };

            match guess_player_two.cmp(&random_number) {
                Ordering::Less => println!("Too small!  \n"),
                Ordering::Greater => println!("Too big! \n"),
                Ordering::Equal => {
                    println!("Player Two wins!");
                    break;
                }
            }
        }
        break;
    }

    println!("Hello, world!");
}
