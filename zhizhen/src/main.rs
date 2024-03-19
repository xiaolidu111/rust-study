// use std::ops::Deref;
// use std::rc::Rc;

// struct MyBox<T> {
//     value: T,
// }
// impl<T> MyBox<T> {
//     fn new(arg: T) -> MyBox<T> {
//         MyBox { value: arg }
//     }
// }
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
// fn hello(str: &str) {
//     println!("{}", str);
// }
// // enum List {
// //     Cons(i32, Box<List>),
// //     Nil,
// // }
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
// use crate::List::Cons;
// use crate::List::Nil;
// fn main() {
//     let x = 5;
//     // let y = &x;
//     let y = MyBox::new(x);

//     println!("{},{}", x, *y);
//     let m = MyBox::new(String::from("value"));
//     hello(&m);

//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("{}", Rc::strong_count(&a));
//     let d = a.clone();
//     println!("{}", Rc::strong_count(&a));
//     {
//         let b = Cons(3, Rc::clone(&a));
//         println!("{}", Rc::strong_count(&a));
//     }
//     let c = Cons(3, Rc::clone(&a));
//     println!("{}", Rc::strong_count(&a));
// // }
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;
// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     println!("a next item = {:?}", a.tail());
// }

#![allow(unused_variables)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
