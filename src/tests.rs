#[cfg(test)]
mod tests {
    use crate::*;

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
    fn multiple_results() {
        let query = "u";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn no_results() {
        let query = "z";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "p";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "p";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search_case_insensitive(query, contents)
        );
    }
}
