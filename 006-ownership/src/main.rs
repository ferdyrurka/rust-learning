/**
* Variables they are valid until they exceed the range.
*
* :: cal to function in Rust
*
* variables is copy when they are transmitted in arguments function.
*
* & chars to references
*
* Data race is in situation:
*
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.
*
*
*/

fn main() {
    let mut string_var = String::from("Rust ");
    string_var.push_str("is very interesting");

    println!("string_var value, create from String::from : {}", string_var);

    //Bug, not call to function clone
    //let s1 = String::from("Bug");
    //let s2 = s1;

    let s1 = String::from("Bug");
    let s2 = s1.clone();

    println!("s1: {} s2: {}", s1, s2);

    copy_fn(s2);

    println!("Function return_string return: {}", return_string());

    let (str_var, i8_var) = multi_return();

    println!("Function multi_return return string value: \"{}\" and i8 value: {}", str_var, i8_var);

    println!("******************* References *******************");

    let mut string_add_var = String::from("INIT ");

    println!("string_add_var value before func add_string in main function: {}", string_add_var);

    add_string(&mut string_add_var);

    //return "string_add_var value in main function: INIT HELLO WORLD"
    println!("string_add_var value after func add_string in main function: {}", string_add_var);

    //let s1 = &string_add_var; // no problem
    //let s2 = &string_add_var; // no problem
    //let s3 = &mut string_add_var; // Bug
}

fn copy_fn(s: String) {
    println!("{}", s);
}

fn return_string() -> String {
    let string_var = String::from("Return value in function return_string");

    // First method return
    // string_var

    // Second method return
    return string_var;
}

fn multi_return() -> (String, i8) {
    ("Hello World".to_string(), 32)
}

fn add_string(string_add_var: &mut String) {
// Bug
// fn add_string(string_add_var: &String) {
// fn add_string(string_add_var: String) {
    string_add_var.push_str("HELLO WORLD");

    println!("Value add string is: {}", string_add_var);
}