use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Missing query"),
        };

        let file_path = match args.next() {
            Some(fp) => fp,
            None => return Err("Missing file path"),
        };

        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }
}
