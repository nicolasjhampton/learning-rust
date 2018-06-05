use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_name: &'a String,
    pub case_sensitive: bool,
}


impl<'a> Config<'a> {
    pub fn new(args: &Vec<String>) -> Result<Config, Box<Error>> {
        
        let query = args.get(1).ok_or("missing query argument")?;
        let file_name = args.get(2).ok_or("missing filename argument")?;

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.file_name)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

// pub fn search(query: &String, text: &String) -> Result<String, String> {
//     //let mut contents = String::new();
//     let text = io::Cursor::new(text);
//     // let f = BufReader::new(f);

//     for line_wrap in text.lines() {
//         if let Ok(line) = line_wrap {
//             if line.contains(query) {
//                 return Ok(line)
//             }
//         }
//     }

//     Err(String::from("query not found"))
// }

// pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> { // see how the lifetime of the text is matched to the lifetime of the result?
//     for line in text.lines() {
//         if line.contains(query) {
//             return vec![line]
//         }
//     }
//     vec![]
// }

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> { // see how the lifetime of the text is matched to the lifetime of the result?

    let mut results = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> { // see how the lifetime of the text is matched to the lifetime of the result?

    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in text.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_result() {
        let query = String::from("duct");
        let content = String::from("\
Rust:
safe, fast, productive.
Pick three.");

        // assert_eq!(
        //     String::from("safe, fast, productive."),
        //     search(&query, &content).unwrap()
        // );
        assert_eq!(
            vec!["safe, fast, productive."],
            search(&query, &content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        // assert_eq!(
        //     String::from("safe, fast, productive."),
        //     search(&query, &content).unwrap()
        // );
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, content)
        );
    }
}
