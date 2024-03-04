fn main() {
    println!("Hello, world!");
    let mut s = "nihao".to_string();
    let s1 = String::from("hahah");
    s.push_str(&s1);
    println!("{}", s1);
    let s2 = String::from("123");
    let s3 = String::from("1234");
    let s4 = String::from("1235");
    let s5 = format!("{}-{}-{}", &s2, &s3, s4);
    println!("{},{},{},{}", s2, s3, s4, s5);
    let s6 = String::from("你好").len();
    println!("{}", s6);
}
