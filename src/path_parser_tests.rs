#[cfg(test)]
pub mod path_parser_tests {
    use crate::path_parser::path_parser::get_dates_from_path;
    use chrono::{Datelike, NaiveDate};

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

    #[test]
    fn dates_in_yyyy_mm_dd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/2018-01-07/2013-05-03_IMG_14523.jpg");

        assert_eq!(results.len(), 2);

        let first_date = results.iter().find(|date| {
            date.year() == 2018 && date.month() == 1 && date.day() == 7
        }
        );

        assert!(first_date.is_some());

        let second_date = results.iter().find(|date| {
            date.year() == 2013 && date.month() == 5 && date.day() == 3
        }
        );

        assert!(second_date.is_some());
    }

    #[test]
    fn dates_in_yyyy_dot_mm_dot_dd_should_be_extracted() {
        let results = get_dates_from_path("/mnt/pics/2019.10.29/2017.02.11_petya_71.jpg");

        assert_eq!(results.len(), 2);

        assert!(vec_contains_date(&results, 2019, 10, 29));

        let second_date = results.iter().find(|date| {
            date.year() == 2017 && date.month() == 2 && date.day() == 11
        }
        );

        assert!(second_date.is_some());
    }

    fn vec_contains_date(vec: &Vec<NaiveDate>, year: i32, month: u32, day: u32) -> bool {
        let date_found = vec.iter().find(|date| {
                date.year() == year && date.month() == month && date.day() == day
            }
        );

        date_found.is_some()
    }
}
