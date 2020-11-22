pub mod path_parser {
    use chrono::{NaiveDate};
    use regex::Regex;

    pub fn get_dates_from_path(path: &str) -> Vec<NaiveDate> {
        let mut results: Vec<NaiveDate> = Vec::new();

        info!("extract date form path '{}'", path);

        results.extend(
        extract_dates_from_path("(\\d{4}\\d{2}\\d{2})", "%Y%m%d", path)
        );
        results.extend(
        extract_dates_from_path("(\\d{4}-\\d{2}-\\d{2})", "%Y-%m-%d", path)
        );

        results
    }

    fn extract_dates_from_path(pattern: &str, date_format: &str, path: &str) -> Vec<NaiveDate> {
        let mut results: Vec<NaiveDate> = Vec::new();

        let date_pattern = Regex::new(pattern).unwrap();

        for cap in date_pattern.captures_iter(path) {
            let date_str = format!("{}", &cap[1]);

            println!("extracted date: '{}'", date_str);

            match NaiveDate::parse_from_str(&date_str, date_format) {
                Ok(datetime) => results.push(datetime.to_owned()),
                Err(e) => eprintln!("unable to parse string to date: {}", e)
            }
        }

        results
    }
}
