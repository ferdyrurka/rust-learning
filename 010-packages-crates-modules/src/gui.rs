pub struct Button {
    pub label: String,
    //id -> private field
    //id: i32
}

impl Button {
    pub fn draw(&self) {
        println!("Draw button by label: {}", self.label);
    }
}