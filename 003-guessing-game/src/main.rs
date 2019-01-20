/**
* URL: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
* std::io is Input/Output library. In java is io/nio
* use - is identical to import in java use in php
* let - is words to created local variables. Only used let is not changing value variables.
* mut - words giving opportunity to changing value variables.
* io::stdin - is library to input value
* & - type identical to C/C++. & convey references. "&" not save repeatedly copy to memory.
*/

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    /**
    * Not use mut in variable
    */
    let variable = "Hello World".to_string();

    println!("let variable value is: {}", variable);
}
