// fn main() {
//     let s = [1, 2, 3, 4, 5, 6, 7, 8, 1, 111, 90, 34];
//     let max_value = get_max(&s);
//     println!("最大的值是:{}", max_value);
//     // let s = vec![2, 4, 1, 3, 6, 7, 9, 8, 30];
//     let s = vec!['a', 'b', 'd', 'c', 't', 'f', 'h', 's'];
//     let max_value = get_max(&s);
//     println!("最大的值是:{}", max_value);
//     let t1 = Point { x: 1, y: 2 };
//     let t2 = Point { x: 1.0, y: 2 };
// }
// fn get_max<T>(s: &[T]) -> T {
//     let mut inital_val = s[0];
//     for &item in s {
//         if item > inital_val {
//             inital_val = item
//         }
//     }
//     inital_val
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// use fanxing::{Summary, Tweet};

// fn main() {
//     let tweet = Tweet {
//         username: String::from("xiaolidu"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     };
//     println!("{}", tweet.summarize())
// }

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > &largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let string_list = vec![String::from("value"), String::from("111")];
    let result = largest(&string_list);
    println!("The largest char is {}", result);
}
