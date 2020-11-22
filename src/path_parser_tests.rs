#[cfg(test)]
pub mod path_parser_tests {
    use crate::path_parser::path_parser::get_dates_from_path;
    use chrono::{Datelike, NaiveDate};

    #[test]
    fn dates_in_yyyymmdd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/20191123/20190527_IMG_14523.jpg");

        assert_eq!(results.len(), 2);

        assert!(vec_contains_date(&results, 2019, 11, 23));
        assert!(vec_contains_date(&results, 2019, 5, 27));
    }

    #[test]
    fn dates_in_yyyy_mm_dd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/2018-01-07/2013-05-03_IMG_14523.jpg");

        assert_eq!(results.len(), 2);

        assert!(vec_contains_date(&results, 2018, 1, 7));
        assert!(vec_contains_date(&results, 2013, 5, 3));
    }

    #[test]
    fn dates_in_yyyy_dot_mm_dot_dd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/2019.10.29/2017.02.11_petya_71.jpg");

        assert_eq!(results.len(), 2);

        assert!(vec_contains_date(&results, 2019, 10, 29));
        assert!(vec_contains_date(&results, 2017, 2, 11));
    }

    #[test]
    fn invalid_dates_should_be_ignored() {
        let results = get_dates_from_path("/mnt/pics/20196229/2019.62.49/2017-02-99_IMG13.jpg");
        assert_eq!(results.len(), 0);
    }

    fn vec_contains_date(vec: &Vec<NaiveDate>, year: i32, month: u32, day: u32) -> bool {
        let date_found = vec.iter().find(|date| {
                date.year() == year && date.month() == month && date.day() == day
            }
        );

        date_found.is_some()
    }
}
