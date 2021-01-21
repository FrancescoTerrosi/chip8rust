mod chip8;

use std::io;
use std::any::type_name;
use rand::Rng;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Hello world!");

    loop {

        println!("Choose:\n p to play\n q to quit\n");

        let mut play = String::new();
        io::stdin().read_line(&mut play);


        play = String::from(play.trim());


        match play.as_str() {
            "p" => println!("OK!"),
            "q" => {
                println!("RIP!");
                break;
        }
            _ => {
                println!("P4N1K!");
                continue;
            }
        };


        let mut guess = String::new();
        let mut victory = false;

        let number_to_guess = rand::thread_rng().gen_range(1..101);

        while !victory {
            guess = "".to_string();
            println!("Guess a number: ");
            io::stdin()
                .read_line(&mut guess)
                .expect("OH NO JOHN!");


            let mut guess_to_int:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("Yor guess is: {}", guess);


            match guess_to_int.cmp(&number_to_guess) {
                std::cmp::Ordering::Less => println!("Too small!"),
                std::cmp::Ordering::Greater => println!("Too big!"),
                std::cmp::Ordering::Equal => {
                    victory = true;
                    println!("Too right")
                },
            }
        }
    }
}