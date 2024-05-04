//https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
//https://doc.rust-lang.org/book/ch04-03-slices.html
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn fibonnaci (n :u64, result: &u64) -> u64 {

    if n <= 2 {
        *result = 1;
        return *result;
    }
    else {
        *result = fibonnaci(n -1, result) + fibonnaci(n -2, result);
        return *result;
    }
}


fn main() {
    let mut result : u64;
    fibonnaci(42, &result);
    println!("Number is {}",  result);
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






