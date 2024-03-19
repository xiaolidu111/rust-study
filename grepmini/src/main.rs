use std::{env, process};

use grepmini::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("程序出现错误,{}", err);
        process::exit(1);
    });
    if let Err(e) = run(&config) {
        println!("读取文件失败{}", e);
        process::exit(2);
    }
}
