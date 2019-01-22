/**
* Variables default in Rust is immutable.
* constant declared by word const. Constant is immutable. Must have a type value.
*/

fn main() {
    const MAX_USER: i32 = 1000;
    println!("Max user constant value is: {}", MAX_USER);

    let x = 5;
    let y = x * 10;

    println!("Y = {}", y);

    //Bug - error compilation
    //let x = 5;
    //x = 10;

    //Integer 32 bit
    let convert: i32 = 8;

    println!("convert (Integer) value is: {}", convert);

    //String
    let convert = "Hello World".to_string();

    println!("convert (String) value is: \"{}\"", convert);

    //Bug -> can not convert immutable to not immutable
    //let x: i32 = 8;
    //let mut x = "Hello World".to_string();;
}