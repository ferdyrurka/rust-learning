/**
* Struct is similar to slices.
* Struct are more flexible than slices.
*
* in variables declared struct in this way
// key:value
*
* Variables can not name how function!!!
*
* Call to impl function by ::
*
* To create impl must be is created Struct.
* One Struct can have many impl.
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

    println!("************************ impl ************************");

    let user_register: User = User {
        username: String::from("ferdyrurka"),
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    };

    user_register.register();

    let user_not_active: User  = User {
        active: false,
        ..user_register
    };

    user_not_active.register();

    let user_register_identical = User {
        username: String::from("ferdyrurka"),
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    };

    if user_not_active.identical(&user_register_identical) {
        println!("user_register_identical and user_not_active is identical!");
    }

    let user_register_not_identical = User {
        username: String::from("rurka"),
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    };

    if user_not_active.identical(&user_register_not_identical) {
        println!("user_register_not_identical and user_not_active is identical");
    } else {
        println!("user_register_not_identical and user_not_active is not identical!");
    }

    //Call to impl function
    println!("Square area is: {}", Square::area(3.4));
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

impl User {
    fn register(&self) -> bool {
        if !self.active {
            println!("User is not active!");

            return false;
        }

        println!("Save user by data: Username: {}, Email: {}, Years: {}",
                 self.username, self.email, self.years
        );

        return true;
    }

    //fn set_active(&mut self) - mut variable

    fn identical(&self, user_second: &User) -> bool {
        self.username == user_second.username &&
            self.email == user_second.email &&
            self.years == user_second.years
    }
}

struct Square(u32);

impl Square {
    fn area(a: f32) -> f32 {
        a * a
    }
}

struct Point(isize, isize);