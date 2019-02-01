/**
* Struct is similar to slices.
* Struct are more flexible than slices.
*
* in variables declared struct in this way
// key:value
*
*
*
*
*
*
*
*
*
*
*/

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
}

fn get_user(username: String) -> User {
    User {
        username,
        years: 20,
        email: String::from("kontakt@lukaszstaniszewski.pl"),
        active: true
    }
}

struct User {
    username: String,
    email: String,
    years: i16,
    active: bool,
}

struct Point(isize, isize);