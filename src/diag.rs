pub mod diag {
    use std::io;
    use crate::files::files::get_files_from_path;
    use std::path::Path;
    use crate::path_parser::path_parser::get_dates_from_path;
    use crate::exif::exif::get_date_created_from_file_exif;

    pub fn diag_path(src_path: &str) -> Result<Vec<String>, io::Error> {
        info!("path '{}' diagnostics", src_path);

        match get_files_from_path(src_path) {
            Ok(files) => {
                let mut results: Vec<String> = Vec::new();

                for (_, file_path_str) in files.iter().enumerate() {
                    info!("processing file '{}'", file_path_str);

                    match get_date_created_from_file_exif(&file_path_str) {
                        Ok(date_created) => {
                            match date_created {
                                Some(_) => {}
                                None => {
                                    let extracted_dates = get_dates_from_path(&file_path_str);

                                    if extracted_dates.is_empty() {
                                        info!("added '{}'", file_path_str);
                                        results.push(String::from(file_path_str))
                                    }
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }

                Ok(results)
            }
            Err(e) => {
                error!("unable to get files from path '{}': {}", src_path, e);
                Err(e)
            }
        }
    }
}
