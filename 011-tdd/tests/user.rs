//#[cfg_test]
// Only use in other tests directory
// cargo test --test [file name]
// Unit tests only use in src files
// Integration tests they should be in tests directories

use tdd::user::User;
use rand::random;

#[test]
pub fn failed_save_user() {
    let user:User = User {
        username: String::from("FAILED"),
        id: random()
    };

    assert!(!user.save());
}

#[test]
pub fn success_save_user() {
    let user:User = User {
        username: String::from("Ferdyrurka"),
        id: random()
    };

    assert!(user.save());
}