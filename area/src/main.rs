#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let square1 = Rectangle::square(20);
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("{}", rect.get_area());
    println!("{:#?}", rect);
    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));
    println!("{:#?}", square1);
}

// fn get_area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
