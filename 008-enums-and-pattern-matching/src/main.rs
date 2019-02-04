/**
* Enum same how struct perhaps created impl
*
*
*
*
*
*
*
*/

fn main() {
    //route(IpAddrKind::IPV4(192, 168, 1, 1);
    //route(IpAddrKind::IPV6(String::from("::1")));

    let messages: Messages = Messages::CTX;
    messages.write_quit();

    // Problem null value
    let some_number = Some(5);
    let absent_number : Option<i32> = None;
    let absent_number_second : Option<i32> = Some(24);
}

enum IpAddr {
   IPV4,
   IPV6
}

enum IpAddrKind {
    IPV4(u8, u8, u8, u8),
    IPV6(String)
}

// Enum vs Struct

enum Messages {
    Quit {message: String},
    Hello(String),
    CTX,
}

struct QuitMessage {
    message: String
}
struct HelloMessage (String);
struct CTX;



impl Messages {
    fn write_quit(&self) {
        println!("Write_quit in Messages impl")
    }
}