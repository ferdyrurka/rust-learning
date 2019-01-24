/**
* Variables default in Rust is immutable.
* constant declared by word const. Constant is immutable. Must have a type value.
*
* Type variables (Integer) declarated by to chars 'u[byte]' and 'i[byte]' :
* 8-bit	    i8  	u8
* 16-bit	i16	    u16
* 32-bit	i32	    u32
* 64-bit	i64	    u64
* 128-bit	i128	u128
* arch	    isize	usize (64 or 32 bit, depends on architecture architecture OS)
*
* Type variables (float\double) declarated by to char 'f[byte]':
* Identical to Integer
*
* Type variables (boolean) declarated by to word 'bool':
*
* Char is declarated single quotes, string is declarated double quotes
* Example:
// let char: char = 'a';
// let string: = "Hello World";
*
* Type variables tuple, it's similar to array. Declarated in type variables type value
* (differs from the array in that the tuple can have different types of values).
* Get value it's similar to array, first index is 0.
*** First way
// let tup: (i8, char, u32) = (1, 'H', 32);
*** Second way
// let tup = (10, 3,2, 2);
// let (x, y, z) = tup;
*
* Array in rust must declarated length array and type value. First index is 0.
// let array: [i8; 2] = [1, 2];
*** get value from array
// array[0];
*
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
    //let mut x = "Hello World".to_string();

    // Tuple

    let tup: (i8, char, u32) = (1, 'H', 32);

    println!("Tup first value is: {}", {tup.0});
    println!("Tup second value is: {}", {tup.1});
    println!("Tup three value is: {}", {tup.2});

    //Is bug not convert to string
    //println!("Tup all: {}", tup);

    // Array

    let array: [i8; 2] = [1, 2];

    println!("Array first value is: {}", array[0]);
    println!("Array second value is: {}", array[1]);

    //Security in rust give check exist index in array. Example code for bug in compilation.
    //let array: [i8; 2] = [1, 2];
    //println!("Array first value is: {}", array[10]);
}