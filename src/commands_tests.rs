#[cfg(test)]
pub mod commands_tests {
    use crate::commands::commands::reorganize_files;
    use std::fs;
    use std::path::Path;
    use crate::logging::logging::get_logging_config;

    const RESULTS_DIR_NAME: &str = "results";

    #[test]
    fn result_filename_should_be_stored_in_year_directory() {
        let logging_config = get_logging_config("debug");
        log4rs::init_config(logging_config).unwrap();

        remove_results_dir();

        reorganize_files("img-src", "results");

        let expected_result_filename = format!("{}/2020/Октябрь/2020-10-10__12-09-47__IMG_20201010_120947.jpg", RESULTS_DIR_NAME);
        let expected_result_file = Path::new(&expected_result_filename);

        assert!(expected_result_file.exists());
    }

    fn remove_results_dir() {
        let results_path = Path::new(RESULTS_DIR_NAME);

        if results_path.exists() {
            fs::remove_dir_all(RESULTS_DIR_NAME);
        }
    }

}
