#[derive(Debug)]
enum Spreadsheetcell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    v.push(1);
    v1.push(2);
    let v2 = vec![1, 2, 3, 4, 5];
    let third = &v2[1];
    println!("{}", third);
    match v2.get(102) {
        Some(one) => {
            println!("{}", one);
        }
        None => {
            println!("there is no element");
        }
    };
    let mut v3 = vec![12, 3, 4, 5, 5, 6];
    let first = &mut v3[0];
    println!("{}", first);
    let mut v4 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for i in &mut v4 {
        *i += 10;
    }
    for i in v4 {
        println!("{}", i);
    }
    let row = vec![
        Spreadsheetcell::Int(1),
        Spreadsheetcell::Float(0.1),
        Spreadsheetcell::Text(String::from("nihao")),
    ];
    println!("{:#?},{:#?},{:#?}", row[1], row[2], row[0]);
    for v in &row {
        println!("{:#?}", v);
    }
}
