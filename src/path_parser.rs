pub mod path_parser {
    use chrono::{NaiveDate};
    use regex::Regex;
    use std::path::PathBuf;

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

    pub fn get_path_without_dir_names(path: &str, skip_folder_names: &Vec<String>) -> String {
        let path_separator = std::path::MAIN_SEPARATOR;

        let file_path_parts: Vec<&str> = path.split(path_separator).collect();

        let mut new_path: PathBuf = PathBuf::new();

        for file_part in file_path_parts.iter() {
            let file_part_in_lowercase = file_part.to_lowercase();

            let mut pattern_found = false;

            for folder_name in skip_folder_names.iter() {
                let mask_in_lowercase = folder_name.to_lowercase();
                if file_part_in_lowercase.starts_with(&mask_in_lowercase) {
                    pattern_found = true;
                    break;
                }
            }

            if !pattern_found {
                new_path = new_path.join(file_part);
            }
        }

        let sanitized_path: &str = new_path.to_str().unwrap_or("");

        String::from(sanitized_path)
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
