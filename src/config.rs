pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

   pub fn get_file_path(&self) -> &String {
       &self.file_path
   }
}
