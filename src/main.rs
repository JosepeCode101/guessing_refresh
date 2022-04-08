// Refrescando conocimientos de rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    // println!("Hello, world!");

    println!("Guess the number");

    let secret_num = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("You guessend: {}", guess);


        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Secret number = {}", secret_num);
    
    
        match guess.cmp(&secret_num) {

            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Y O U    W I N");
                break;
            }

        }
    }


}

