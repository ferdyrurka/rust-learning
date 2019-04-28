use tdd::user::User;
use rand::random;

fn main() {
    let user:User = User{
        username: String::from("Ferdyrurka"),
        id: random()
    };

    if user.save() {
        println!("Success save");

        return;
    }

    println!("Unsuccessful save");
}
