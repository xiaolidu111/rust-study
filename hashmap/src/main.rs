use std::{collections::HashMap, hash};

fn main() {
    println!("Hello, world!");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("语文"), 32);
    scores.insert(String::from("数学"), 54);
    scores.insert(String::from("英语"), 65);
    let teams = vec![String::from("中国"), String::from("英国")];
    let scoresv = vec![32, 54];

    let mut scores_country: HashMap<_, _> = teams.iter().zip(scoresv.iter()).collect();
    let american = String::from("美国");
    let american_score = 32;
    scores_country.insert(&american, &american_score);
    let china = String::from("中国");
    let score = scores_country.get(&china);
    match score {
        Some(strs) => {
            println!("{}", strs);
        }
        None => {
            println!("None");
        }
    }
    for (key, value) in &scores_country {
        println!("{}:{}", key, value);
    }
    let new_score = 33;
    let russian = String::from("俄罗斯");
    let russian_score = 39;
    scores_country.insert(&american, &new_score);
    // scores_country.insert(&russian, &russian_score);
    let e = scores_country.entry(&russian);
    e.or_insert(&russian_score);
    let new_american_score = 159;
    let new_american = String::from("美国");
    scores_country
        .entry(&new_american)
        .or_insert(&new_american_score);
    println!("{:?}", scores_country);
    let word = "你好 我好 大家好 你好";
    let mut word_map = HashMap::new();

    for w in word.split_whitespace() {
        let count = word_map.entry(String::from(w)).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_map);
}
