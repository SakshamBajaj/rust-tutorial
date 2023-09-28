use std::io;
use std::fs;
use std::env;

pub struct Config {
    query:  String,
    filename:  String,
    case_sensitive: bool
}


impl Config{
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // eat the first argumetn
        args.next();
        // get the query and filename
        let query = args.next().unwrap_or_else(|| "No query provided".to_string());
        let filename = args.next().unwrap_or_else(|| "No filename provided".to_string());

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        Ok(Config { query: query, filename: filename, case_sensitive: case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // read file into variable
    let file_contents = read_file(&config.filename)?;
    let matches: Vec<&str>;
    // search file contents for query
    if config.case_sensitive{
        matches = search(&config.query, &file_contents);

    }
    else {
        matches = search_case_insensitive(&config.query, &file_contents);
    }
    for res in matches.iter() {
        println!("{}", res);
    }
    Ok(())
}
// fn to read file contents w/ error handling
fn read_file(filename: &str) -> io::Result<String> {
    let file_contents = fs::read_to_string(filename)?;
    Ok(file_contents)
}

fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents.lines().filter(|line| line.contains(query)).collect()
    
}

fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    search(&query, file_contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_test() {
        let query = "test-query";
        let file_contents = "\
        Here are some testcontents
a line containing test-query
        ";
        assert_eq!(search(query, file_contents), vec!["a line containing test-query"]);
    }
}