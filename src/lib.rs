use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

/// Some variables
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 'new' can not failed, use 'from/build' instead
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let (query, file_path) = Self::get_query_and_file_path(args);

        // let ignore_case = env::var("IGNORE_CASE").map_or(false, |var| var.eq("1"));
        let ignore_case = Self::check_env("IGNORE_CASE")
            || Self::check_options(args, &[String::from("-i"), String::from("--ignore-case")]);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // NOTE: A rusty example
    //
    // pub fn from(args: &[String]) -> Result<Self, &'static str> {
    //     let command: Vec<&String> = args.iter().filter(|arg_str|!arg_str.starts_with("-")).collect();
    //     if command.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //
    //     let query = command[1].clone();
    //     let file_path = command[2].clone();
    //
    //     // 查询是否忽略大小写
    //     // 优先找环境变量 IGNORE_CASE
    //     // 没有 IGNORE_CASE 环境变量，再查找命令行参数
    //     let ignore_case = match env::var("IGNORE_CASE") {
    //         Ok(flag) => flag != "0",
    //         Err(_) => args.iter().any(|v| v == "-i" || v == "--ignore_case"),
    //     };
    //
    //     Ok(Config{query, file_path, ignore_case})
    // }

    /// query: the first non-option argument
    /// file_path: the second non-option argument
    pub fn get_query_and_file_path(args: &[String]) -> (String, String) {
        let mut query = String::new();
        let mut file_path = String::new();

        for arg in &args[1..] {
            if Self::is_option(arg) {
                continue;
            }
            if query.is_empty() {
                query = String::from(arg);
            } else {
                file_path = String::from(arg);
                break;
            }
        }

        (query, file_path)
    }

    /// Options are longer than 1 with - prefix
    /// Example: -i --ignore-case
    pub fn is_option(arg: &str) -> bool {
        arg.len() > 1 && arg.starts_with('-')
    }

    /// Check whether specific arguments exist or not
    pub fn check_options(args: &[String], targets: &[String]) -> bool {
        for target in targets {
            if args[1..].contains(target) {
                return true;
            }
        }
        false
    }

    // TODO:
    /// Check options and get there values, reture key-values
    pub fn check_options_and_get_value(_target: &str) -> Option<HashMap<String, String>> {
        todo!()
    }

    /// Environment Varibles
    /// Return true if it is not empty or non-zero
    pub fn check_env(variable: &str) -> bool {
        let flag = env::var(variable).ok();
        !matches!(flag.as_ref().map(String::as_ref), None | Some("0"))
    }
}

// Box<dyn Error>>: Trait object of Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Return lines with specific query of contents, case sensitive
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Return lines with specific query of contents, case sensitive
pub fn _search2<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    contents
        .lines()
        .filter(|line| line.contains(query))
        .for_each(|line| results.push(line.trim()));
    results
}

/// Return lines with specific query of contents, case insensitive
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// NOTE: 测试驱动开发模式(TDD, Test Driven Development)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
