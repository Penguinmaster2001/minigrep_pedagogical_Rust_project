
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}
