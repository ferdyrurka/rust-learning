use oop_features::gui::screen::Screen;
use oop_features::gui::components::alert::Alert;
use oop_features::gui::draw::Draw;
use oop_features::gui::components::button::Button;

struct Input {
    name: String,
    placeholder: String
}

impl Draw for Input {
    fn draw(&self) {
        println!("Input draw by name: {} and placeholder: {}", self.name, self.placeholder);
    }
}

fn main() {
    let screen:Screen = Screen {
        components: vec![
            Box::new(Input {
                name: String::from("nickname"),
                placeholder: String::from("Your nickname")
            }),
            Box::new(Button {
                label: String::from("Save")
            }),
            Box::new(Alert {
                message: String::from("Error, undefined nickname ferdyrurka")
            })
        ]
    };

    screen.run();
}