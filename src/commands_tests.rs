#[cfg(test)]
pub mod commands_tests {
    use crate::commands::commands::reorganize_files;
    use std::fs;
    use std::path::Path;
    use crate::logging::logging::get_logging_config;
    use crate::domain::domain::NoExifConfig;
    use crate::get_extension_filters;

    const SOURCE_DIR_NAME: &str = "img-src";
    const RESULTS_DIR_NAME: &str = "results";

    const EXPECTED_FILE_WITHOUT_DATE_IN_EXIF: &str = "/2013/Май/2013-05-17__manga__berserk__forest.jpg";

    #[test]
    fn date_should_extracted_from_path_for_files_without_exif() {
        remove_results_dir();

        let no_exif_config = NoExifConfig {
            extract_dates_from_path: true,
            skip_dir_names_for_date_extract: Vec::new(),
            force_year: false,
            year: 0
        };

        let ext_filters = get_extension_filters();

        match reorganize_files(SOURCE_DIR_NAME, RESULTS_DIR_NAME,
                               &ext_filters, &no_exif_config, show_progress) {
            Ok(_) => {}
            Err(_) => {}
        }

        let expected_result_filename = format!("{}/{}", RESULTS_DIR_NAME, EXPECTED_FILE_WITHOUT_DATE_IN_EXIF);
        let expected_result_file = Path::new(&expected_result_filename);

        assert!(expected_result_file.exists());
    }

    #[test]
    fn result_filename_should_be_stored_in_year_directory() {
        remove_results_dir();

        let no_exif_config = NoExifConfig {
            extract_dates_from_path: false,
            skip_dir_names_for_date_extract: Vec::new(),
            force_year: false,
            year: 0
        };

        let ext_filters = get_extension_filters();

        match reorganize_files(SOURCE_DIR_NAME, RESULTS_DIR_NAME,
                               &ext_filters, &no_exif_config, show_progress) {
            Ok(_) => {}
            Err(_) => {}
        }

        let expected_result_filename = format!("{}/2020/Октябрь/2020-10-10__12-09-47__IMG_20201010_120947.jpg", RESULTS_DIR_NAME);
        let expected_result_file = Path::new(&expected_result_filename);

        assert!(expected_result_file.exists());
    }

    #[test]
    fn do_not_extract_date_from_path_for_files_without_exif_if_option_was_not_activated() {
        let logging_config = get_logging_config("debug");
        log4rs::init_config(logging_config).unwrap();

        remove_results_dir();

        let no_exif_config = NoExifConfig {
            extract_dates_from_path: false,
            skip_dir_names_for_date_extract: Vec::new(),
            force_year: false,
            year: 0
        };

        let ext_filters = get_extension_filters();

        match reorganize_files(SOURCE_DIR_NAME, RESULTS_DIR_NAME,
                               &ext_filters, &no_exif_config, show_progress) {
            Ok(_) => {}
            Err(_) => {}
        }

        let expected_result_filename = format!("{}{}", RESULTS_DIR_NAME, EXPECTED_FILE_WITHOUT_DATE_IN_EXIF);
        let expected_result_file = Path::new(&expected_result_filename);

        assert!(!expected_result_file.exists());
    }

    fn remove_results_dir() {
        let results_path = Path::new(RESULTS_DIR_NAME);

        if results_path.exists() {
            match fs::remove_dir_all(RESULTS_DIR_NAME) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    fn show_progress(total_elements: usize, current_element_index: usize) {
        print!("\r");
        print!("progress: {}/{}", current_element_index, total_elements);
    }

}
