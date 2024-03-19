#[derive(Debug)]
enum ipAddrKid {
    v4(u8, u8, u8, u8),
    v6(String),
}
#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call() {
        println!("{:#?}", Message::Move { x: 11, y: 22 });
    }
}
fn main() {
    let four = ipAddrKid::v4(0, 0, 0, 0);
    let six = ipAddrKid::v6(String::from("hello world"));
    println!("{:?},{:?}", four, six);
    route(four);
    route(six);
    let quit = Message::Quit;
    Message::call();
}
fn route(ip_kid: ipAddrKid) {}
