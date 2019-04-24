use packages_crates_modules::gui::Button;

/**
* crate or self - ./ - load modules
* super - ../ - load modules
* lib.rs and main.rs is default to load modules
* alternative to create modules is
* mod Button { // Logic }
* default field in struct or enums is private
*
* /// rename name class
*
* use std::fmt::Result;
* use std::io::Result as IoResult;
*
* fn function1() -> Result {
* }
* fn function2() -> IoResult<()> {
* }
*
* pub use packages_crates_modules::gui::Button; - declared use Button everywhere to load main modules
*/

fn main() {
    let button:Button = Button{
        label: String::from("Hello")
    };

    button.draw();
}
