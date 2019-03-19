use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("--- Guess Game! ---");

    let secret = rand::thread_rng().gen_range(1, 11);

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");

        stdin().read_line(&mut guess).expect("Failed to read line");

        if guess.trim() == "x" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Your guess {} is too small.", guess),
            Ordering::Greater => println!("Your guess {} is too big.", guess),
            Ordering::Equal => {
                println!("Your guess {} is correct.", guess);
                break;
            }
        };
    }

    println!("--- Game Ends ---");
}
