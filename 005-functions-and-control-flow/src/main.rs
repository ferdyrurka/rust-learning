/**
* in function arguments must declarated
// fn name(name_variable: type_variable)
*
* In let statement block code is must set one type value.
*
* loop in Rust execute forever or until you explicitly tell it to stop.
*
* In rust does not exist incrementation (++i | i++ | --i | i--).
* You can use it alternative i += 1
*
****Assertion
* assert_eq!(excepted, return) // assert equals
*
* Range type (start_int..end_int).rev()
*/

fn main() {
    function_arguments(5);

    //Expression in rust

    //let x = 10;
    //Does not cause override or modification variable x in expression.

    let y: isize = {
        let x = 5;
        x + 10
        // x + 10; -> bug
        // note: expected type `isize`
        //              found type `()`
    };

    println!("Expression value is: {}", y);

    //if, else, else if

    control_flow_bool(true);
    control_flow_bool(false);

    in_let_statement(3);
    in_let_statement(2);

    //Loop

    loop_rust();
    foreach();
}

fn function_arguments(a: isize) {
    println!("Arguments \"a\" value is: {}", a);
}

fn control_flow_bool(bool_variables: bool) {

    if bool_variables {
        println!("control_flow_bool, value arguments is true");
    } else {
        println!("control_flow_bool, value arguments is false");
    }

}

fn in_let_statement(x: i8) {
    let y = if x > 2 {
        "x arguments is greater than 2"
    } else {
        "x arguments is less than 2"
    };

    println!("in let statement y value is: {}", y);
}

fn loop_rust() {
    let mut i : i8 =0;

    println!("********* loop *********");

    //loop

    loop {
        if i >= 3 {
            break;
        }

        println!("loop in rust, i value is: {}", i);

        i += 1;
    }

    println!("********* while *********");

    //While

    i = 0;

    while i == 1 {
        println!("while in rust, i value is: {}", i);

        i += 1;
    }
}

fn foreach() {
    let array: [i32; 3] = [1, 58, 67];

    println!("*********Foreach in rust.*********");

    for value in array.iter() {
        println!("Value array: {}", value)
    }

    println!("*********Foreach and Range type.*********");

    //Range type
    for number in (1..4).rev() {
        println!("Value range type: {}", number)
    }

    println!("*********List range type is the end*********");
}