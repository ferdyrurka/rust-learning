/**
* Struct is similar to slices.
* Struct are more flexible than slices.
*
* in variables declared struct in this way
// key:value
*
* Variables can not name how function!!!
*
*
*
*
*
*
*
*
*/

#[derive(Debug)]
struct Triangle {
    a: f32,
    h: f32
}

fn main() {
    //They do not have in identical order.
    let user = User {
        username: String::from("ferdyrurka"),
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    };

    let mut user_mut = User {
        username: String::from("ferdyrurka"),
        years: 19,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    };

    user_mut.years = 20;

    let user_factory: User = get_user(String::from("ferdyrurka"));

    println!("variable user_factory value is: Username is: {} Email is: {} Years is: {}",
             user_factory.username, user_factory.email, user_factory.years
    );

    //Other fields will be set by variable userFactory
    let user_factory_2: User = User {
        years: 30,
        active: false,
        ..user_factory
    };

    println!("variable user_factory_2 value is: Username is: {} Email is: {} Years is: {}",
             user_factory_2.username, user_factory_2.email, user_factory_2.years
    );

    let point: Point = Point(1, 2);

    println!("Point value is: x: {}, y: {}", point.0, point.1);

    //First method

    let triangle = Triangle {
        a: 10.2,
        h: 5.8
    };

    println!("Triangle field in first method is: {}", triangle_field(triangle));

    //Second method

    let triangle2= (10.2, 5.3);

    println!("Triangle field in second method is: {}", triangle_field2(triangle2));

    // Bug not convert struct to string
    //println!("triangle value is: {}", triangle);

    let triangle_debug = Triangle {
        a: 23.2,
        h: 56.8
    };

    // No addition #[derive(Debug)] in up struct Rust compiler not compile project because
    // this method is only to debug.
    //
    // {:?} Result is:
    // triangle value is: Triangle { a: 23.2, h: 56.8 }
    //
    // {:#?} Result is:
    // triangle value is: Triangle {
    //    a: 23.2,
    //    h: 56.8
    // }
    //
    println!("triangle value is: {:#?}", triangle_debug);
}

fn get_user(username: String) -> User {
    User {
        username,
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    }
}

fn triangle_field(triangle: Triangle) -> f32 {
    0.5 * triangle.a * triangle.h
}

fn triangle_field2(triangle: (f32, f32)) -> f32 {
    0.5 * triangle.0 * triangle.1
}

struct User {
    username: String,
    email: String,
    years: i16,
    active: bool,
}

struct Point(isize, isize);