enum Coin {
    Penny,
    Nucker,
    Dime,
    Quarter(State),
}
#[derive(Debug)]
enum State {
    alasijia,
    ribewn,
}
fn main() {
    let coin = Coin::Quarter(State::ribewn);
    println!("{}", match_coin(coin));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:#?},{:#?},{:#?}", five, six, none);
    let n = 1u8;
    let n1 = match n {
        1 => "first",
        2 => "second",
        _ => "all",
    };
    println!("{}", n1);
    let v = Some(2);
    if let Some(3) = v {
        println!("匹配成功{:#?}", v);
    } else {
        println!("匹配不成");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn match_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nucker => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            25
        }
    }
}
