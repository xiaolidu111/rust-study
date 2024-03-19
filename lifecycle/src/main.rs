// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let x = 5;
//     let r;
//     {
//         r = &x;
//     }
//     println!("{}", r);
//     // let string1 = String::from("abcd");
//     // let string2 = "xyz";

//     // let result = longest(string1.as_str(), string2);
//     // println!("The longest string is {}", result);
//     let string1 = String::from("long string is long");
//     let result;
//     let string2 = String::from("xyz");
//     {
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// //     if x.len() > y.len() {
// //         x
// //     } else {
// //         y
// //     }
// // }
// fn longest<'a>(x: &str, y: &str) -> String {
//     let result = String::from("really long string");
//     result
// }

fn main() {
    let string1 = String::from("abcd");
    let result;
    let string2;
    {
        string2 = String::from("new");
        result = longest(string1.as_str(), string2.as_str());
    }
    // string2 = String::from("value");
    println!("The longest string is {},{}", result, string2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        y
    } else {
        x
    }
}
