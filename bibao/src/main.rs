use std::{thread, time::Duration};

fn main() {
    // println!("Hello, world!");
    let simulated_user_specified_value = 26;
    let simulated_random_number = 3;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(V) => V,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    };

    let expample_closure = |x| x;

    let s = expample_closure(String::from("nihao"));
    // // let s1 = expample_closure(5);
    // let x = 4;
    // // fn equal_to_x(z: i32) -> bool {
    // //     return z == x;
    // // }
    // let equal_to_x = move |z| z == x;
    // let y = 4;
    // equal_to_x(y);
    // println!("{},{}", equal_to_x(y), x);
    let x = 1;

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = 2;
    equal_to_x(y);
    // assert!(equal_to_x(y));
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let a = c.value(1);
    let b = c.value(2);
    assert_eq!(b, 2);
}
