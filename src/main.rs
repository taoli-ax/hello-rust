use ferris_says::say;
use std::io::{stdout, BufWriter};
use rand::Rng;
use std::io;
use std::cmp::Ordering;


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    println!("Hello, world!");
    hello_rustaceans();
    game_main();
    test_return();
    if_statement();
    build_user("Email@gmal.com".to_string(), "tall".to_string());
}

fn build_user(email: String, username: String) -> User{
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    user1
}


fn hello_rustaceans() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}


fn game_main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
          
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    // let x = 5;
    // let y = 10;

    // println!("x = {x} and y + 2 = {}", y + 2);
}


fn test_return() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1  //不加；代表return
}


fn if_statement() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}