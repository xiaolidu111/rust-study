use std::ops::Deref;
use std::rc::Rc;

// use doc::mix;
// use doc::PrimaryColor;
use crate::List::Cons;
use crate::List::Nil;
fn main() {
    // let red = PrimaryColor::Red;
    // let yellow = PrimaryColor::Yellow;
    // mix(red, yellow);
    // let b = MyBox::new(5);
    // // println!("{}", b);
    // let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let a = 5;
    // let b = MyBox::new(a);

    // assert_eq!(5, a);
    // assert_eq!(5, *b);
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // drop(c);
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a));
    let c = Cons(3, Rc::clone(&a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // let a = String::from("nihao");
    // let b = a;
    // println!("{}", a)
}
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
