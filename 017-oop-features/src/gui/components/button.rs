use crate::gui::draw::Draw;

pub struct Button {
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button with label: {}", self.label);
    }
}
