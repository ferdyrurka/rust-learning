/**
* URL: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
* std::io is Input/Output library. In java is io/nio
* use - is identical to import in java use in php
* let - is words to created local variables. Only used let is not changing value variables.
* mut - words giving opportunity to changing value variables.
* io::stdin - is library to input value
* & - type identical to C/C++. & convey references. "&" not save repeatedly copy to memory.
****
* io library gives in io::Result ("Err" or "Ok"). expect captures Result and in "Err" Result
* output text in arguments (expect function)
****
* {} - replace replaces from arguments
* cargo update -> identical to composer update
* std::cmp::Ordering -> check value (Equal Greater and Less)
* i32 -> 32 bit number
* i64 -> 64 bit number
* Example using type variables let secret_number : i64
* trim() -> trim in PHP
* parse() -> error don't convert value variables. Example variables type integer, value variables is "HEJ%".
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    // Not use mut in variable

    let variable = "Hello World".to_string();

    println!("let variable value is: {}", variable);

    replace_x_and_y();

    rand_integer();

    match_value();
}

fn replace_x_and_y() {
    let mut x= String::new();
    let mut y= String::new();

    println!("Write x value: ");
    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("Write y value: ");
    io::stdin().read_line(&mut y).expect("Failed to read line");

    println!("X={} and Y={}", x.trim(), y.trim());
}

fn rand_integer() {
    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("Secret number is {}", secret_number)
}

fn match_value() {
    let x = rand::thread_rng().gen_range(1, 100);
    let y = rand::thread_rng().gen_range(1, 100);

    println!("X={} and Y={}", x, y);

    match x.cmp(&y) {
        Ordering::Equal => println!("Equal x and y"),
        Ordering::Greater => println!("Greater"),
        Ordering::Less => println!("Less")
    }
}