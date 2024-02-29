use std::io;
fn main(){
    println!("请猜个数！");

    let mut guessNum = String::new();

    io.stdin().read_line(&mut guessNum).expect("无法读取行");

    println!("你猜的数字是：{}",guessNum);
}