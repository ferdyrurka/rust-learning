use crate::gui::draw::Draw;

pub struct Alert {
    pub message: String
}

impl Draw for Alert {
    fn draw(&self) {
        println!("Alert message: {}", self.message)
    }
}