use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("What's your name ?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("No name !");
    // name = name.trim();
    println!("Hello {}, let's play a game shall we ?", name.trim());


    // ret.expect();
    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        println!("You guessed: {}", guess.trim());
        let guess : u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct !");
                break;
            },
        }
    }

    }






