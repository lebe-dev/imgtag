pub mod path_parser {
    use chrono::{NaiveDate};
    use regex::Regex;

    pub fn get_dates_from_path(path: &str) -> Vec<NaiveDate> {
        let mut results: Vec<NaiveDate> = Vec::new();

        info!("extract date form path '{}'", path);

        let date_pattern1 = Regex::new("(\\d{4}\\d{2}\\d{2})").unwrap();

        for cap in date_pattern1.captures_iter(path) {

            let date_str = format!("{}", &cap[1]);

            println!("extracted date: '{}'", date_str);

            match NaiveDate::parse_from_str(&date_str, "%Y%m%d") {
                Ok(datetime) => results.push(datetime.to_owned()),
                Err(e) => eprintln!("unable to parse string to date: {}", e)
            }
        }

        results
    }
}
