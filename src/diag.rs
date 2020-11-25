pub mod diag {
    use std::io;
    use crate::files::files::get_files_from_path;
    use crate::path_parser::path_parser::get_dates_from_path;
    use crate::exif::exif::get_date_created_from_file_exif;

    pub struct DiagReport {
        pub files_total: usize,
        pub files_with_issues: Vec<String>
    }

    pub fn diag_path(src_path: &str, file_ext_filter: &Vec<String>,
             extract_dates_from_path: bool,
             on_progress: fn(total: usize, current_index: usize,
                             with_issue: usize)) -> Result<DiagReport, io::Error> {
        info!("path '{}' diagnostics", src_path);

        match get_files_from_path(src_path, file_ext_filter) {
            Ok(files) => {
                let mut results: Vec<String> = Vec::new();

                for (index, file_path_str) in files.iter().enumerate() {
                    info!("processing file '{}'", file_path_str);

                    match get_date_created_from_file_exif(&file_path_str) {
                        Ok(date_created) => {
                            match date_created {
                                Some(_) => {}
                                None => {

                                    if extract_dates_from_path {
                                        let extracted_dates = get_dates_from_path(&file_path_str);

                                        if extracted_dates.is_empty() {
                                            info!("added '{}'", file_path_str);
                                            results.push(String::from(file_path_str))
                                        }

                                    } else {
                                        info!("added '{}'", file_path_str);
                                        results.push(String::from(file_path_str))
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }

                    on_progress(files.len(), index, results.len())
                }

                Ok(
                    DiagReport { files_total: files.len(), files_with_issues: results }
                )
            }
            Err(e) => {
                error!("unable to get files from path '{}': {}", src_path, e);
                Err(e)
            }
        }
    }
}
