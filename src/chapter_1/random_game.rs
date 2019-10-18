use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn loop_game() {
    loop {
        let secret_number = generate_rand_number(1, 101);
        let mut guess = String::new();
        println!("You guessed: {}", guess);
        get_input_user(&mut guess);
        // let guess: i32 = guess.trim().parse().expect("Should be a number");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!")
        }
    }
}

fn generate_rand_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min, max)
}

fn get_input_user(placeholder: &mut String) {
    println!("Please type a nmber!");
    io::stdin().read_line(placeholder)
        .expect("Failed to read line");
}
