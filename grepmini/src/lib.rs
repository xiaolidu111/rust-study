use std::{env, error::Error, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;
    let result = if config.isqufen {
        search_buqufen_inner(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        eprintln!("{}", line);
    }
    // println!(
    //     "查询参数{}，文件名称，{}，文件内容{}，",
    //     config.query, config.file_name, content
    // );
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_name: String,
    isqufen: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数长度小于3");
        }
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => {
                return Err("参数不够");
            }
        };
        let file_name = match args.next() {
            Some(v) => v,
            None => {
                return Err("参数不够");
            }
        };
        let ishuanjing = env::var("huanjing").is_err();
        println!("{}", ishuanjing);
        Ok(Config {
            query,
            file_name,
            isqufen: ishuanjing,
        })
    }
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(query)).collect()
}
fn search_buqufen_inner<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut result = Vec::new();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    // result
    content
        .lines()
        .filter(|x| x.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_qufen() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    fn search_buqufen() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_buqufen_inner(query, contents)
        )
    }
}
