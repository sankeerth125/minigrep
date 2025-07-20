use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&contents, &config.query)
    } else {
        search_case_insensitive(&contents, &config.query)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path."),
        };

        // Look for --ignore-case in any remaining arguments
        let case_sensitive = !args.any(|arg| arg == "--ignore-case");

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_with_valid_args() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("filename.txt"),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename.txt");
        assert!(config.case_sensitive);
    }

    #[test]
    fn build_config_with_ignore_case() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("filename.txt"),
            String::from("--ignore-case"),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert!(!config.case_sensitive);
    }

    #[test]
    fn search_case_sensitive_match() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(contents, query));
    }

    #[test]
    fn search_case_insensitive_match() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(contents, query)
        );
    }
}
