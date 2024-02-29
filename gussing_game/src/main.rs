use rand::Rng;
use std::cmp::Ordering;
use std::{io, os::unix::net::SocketAddr};
use tokio::runtime::Builder;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("秘密数字: {}", secret_number);

    loop {
        println!("请猜个数！");

        let mut guess_num = String::new();

        io::stdin().read_line(&mut guess_num).expect("无法读取行");
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字！");
                continue;
            }
        };

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
    // let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    // rt.block_on(async {
    //     let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    // });
}
