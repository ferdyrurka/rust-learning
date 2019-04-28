pub struct User {
    pub username: String,
    pub id:u8
}

impl User {
    pub fn save(&self) -> bool {
        println!("I have the data user: username: {} and id: {}", self.username, self.id);

        if self.username != "Ferdyrurka" {
            return false;
        }

        return true;
    }
}

// Here unit tests but I writed in tests directory