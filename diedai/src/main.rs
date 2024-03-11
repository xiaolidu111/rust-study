fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();

    for item in v_iter {
        println!("{}", item);
    }
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), None);
    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let v2_sum: u32 = v2_iter.sum();
    // let v2_sum_1: u32 = v2_iter.sum();
    println!("{:?},{:?}", v2_sum, 6);
    let v3 = vec![1, 2, 3, 4, 5];
    let v3_1: Vec<_> = v3.iter().map(|x| x + 1).collect();
    println!("{:?}", v3_1);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);
    let mut counter = Counter::new();

    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{}", using_other_iterator_trait_methods());
}
fn using_other_iterator_trait_methods() -> u32 {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|item| item % 3 == 0)
        .sum();
    sum
}
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == size).collect()
}
