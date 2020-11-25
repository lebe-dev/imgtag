pub mod path_parser {
    use chrono::{NaiveDate};
    use regex::Regex;

    const SOLID_DATE_FORMAT: &str = "%Y%m%d";
    const SOLID_REGEX_PATTERN: &str = "(\\d{4}\\d{2}\\d{2})";

    const DATE_FORMAT_WITH_HYPHENS: &str = "%Y-%m-%d";
    const DATE_PATTERN_WITH_HYPHENS: &str = "(\\d{4}-\\d{2}-\\d{2})";

    const DATE_FORMAT_WITH_DOTS: &str = "%Y.%m.%d";
    const DATE_PATTERN_WITH_DOTS: &str = "(\\d{4}.\\d{2}.\\d{2})";

    pub fn get_dates_from_path(path: &str) -> Vec<NaiveDate> {
        let mut results: Vec<NaiveDate> = Vec::new();

        info!("extract date form path '{}'", path);

        let date_formats: Vec<(&str, &str)> = vec![
            (SOLID_REGEX_PATTERN, SOLID_DATE_FORMAT),
            (DATE_PATTERN_WITH_HYPHENS, DATE_FORMAT_WITH_HYPHENS),
            (DATE_PATTERN_WITH_DOTS, DATE_FORMAT_WITH_DOTS)
        ];

        for date_format in date_formats.iter() {
            results.extend(
            extract_dates_from_path(date_format.0, date_format.1, path)
            );
        }

        results
    }

    fn extract_dates_from_path(pattern: &str, date_format: &str, path: &str) -> Vec<NaiveDate> {
        let mut results: Vec<NaiveDate> = Vec::new();

        let date_pattern = Regex::new(pattern).unwrap();

        for cap in date_pattern.captures_iter(path) {
            let date_str = format!("{}", &cap[1]);

            info!("extracted date: '{}'", date_str);

            match NaiveDate::parse_from_str(&date_str, date_format) {
                Ok(datetime) => results.push(datetime.to_owned()),
                Err(e) => error!("unable to parse string to date: {}", e)
            }
        }

        results
    }
}
