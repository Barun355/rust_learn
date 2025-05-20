use core::fmt;

use rand::{rng, Rng};
use chrono::{Utc, Local};

mod borrowing;
mod enums;
mod exception;
mod optional;

fn main() {
    // let word: String = String::from("abccba");
    // println!("{}", find_pallindrome(word))

    // let mut my = String::from("hello");

    // borrowing::borrow(&mut my);

    // println!("{}", my)
    //
    // let user: User = create_obj();
    // println!(
    //     "User {} has email {} and there login count is {}",
    //     user.name, user.email, user.login_count
    // );

    // enums::start_enum();

    // let a = 10;
    // let b = 0;
    // let divide = exception::divide(a, b);

    // match divide {
    //     Ok(result) => println!("Division of {} with {} is: {}", a, b, result),
    //     Err(error) => println!("Error occured: {}", error)
    // }

    // let letter = 'a';
    // let sentence = String::from("Testing a file");

    // let index = optional::find_first_occurence(&sentence, letter);

    // match index {
    //     Some(idx) => println!("Index of {} in {} is: {}", letter, sentence, idx),
    //     None => println!("Letter {} is not found in {}", letter, sentence)
    // }

    let mut rng = rng();
    let i: u32 = rng.random();

    println!("{}", i);

    let now = Utc::now();
    println!("Current UTC DateTime {}", now);
    
    let formatted = now.format("%Y-%m-%d");
    println!("Formatted Current UTC DateTime {}", formatted);
    
    let local = Local::now();
    println!("Current  DateTime {}", local);




}

struct User {
    name: String,
    email: String,
    login_count: i64,
    active: bool,
}

fn create_obj() -> User {
    let user = User {
        name: String::from("Pawan Sharma"),
        email: String::from("pawan@gmail.com"),
        login_count: 10,
        active: false,
    };

    return user;
}
