use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut fr = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match fr.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;

//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.tx t")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello, world!");
    // panic!("crush and run");
    // let v = vec![1, 2, 3];
    // v[999];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("error opening file {:?}", error);
    //     }
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => {
    //                 panic!("创建文件失败{:?}", err)
    //             }
    //         },
    //         other_error => {
    //             panic!("其他错误{:?}", other_error)
    //         }
    //     },
    // };
    // let f2 = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("创建文件失败，{:?}", e);
    //             }
    //         },
    //         other_error => {
    //             panic!("其他错误{:?}", other_error);
    //         }
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("无法打开文件 hello.txt");

    // let res = read_username_from_file();
    // println!("{:#?}", res);

    let f = File::open("hello.txt")?;
    Ok(())
}
