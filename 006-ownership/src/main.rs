/**
* Variables they are valid until they exceed the range.
*
* :: cal to function in Rust
*
* variables is copy when they are transmitted in arguments function.
*
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