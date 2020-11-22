#[cfg(test)]
pub mod path_parser_tests {
    use crate::path_parser::path_parser::get_dates_from_path;
    use chrono::Datelike;

    const DATE_COMPARE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    #[test]
    fn dates_in_yyyymmdd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/20191123/20190527_IMG_14523.jpg");

        assert_eq!(results.len(), 2);

        let first_date = results.iter().find(|date| {
                date.year() == 2019 && date.month() == 11 && date.day() == 23
            }
        );

        assert!(first_date.is_some());

        let second_date = results.iter().find(|date| {
                date.year() == 2019 && date.month() == 5 && date.day() == 27
            }
        );

        assert!(second_date.is_some());
    }
}
