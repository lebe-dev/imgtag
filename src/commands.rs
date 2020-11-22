pub mod commands {
    use std::{io, fs};
    use rexif::{ExifError, ExifTag};
    use std::path::Path;
    use chrono::{NaiveDateTime, Datelike};
    use crate::path_parser::path_parser::get_dates_from_path;

    pub fn reorganize_files(src_path: &str, dest_path: &str,
                            extract_dates_from_path: bool) -> Result<(), io::Error> {
        info!("reorganize files for path '{}'", src_path);
        info!("destination path '{}'", dest_path);
        info!("extract dates from path: {}", extract_dates_from_path);

        match get_files(src_path) {
            Ok(files) => {
                for file_path_str in files {
                    info!("processing file '{}'", file_path_str);

                    let file_path = Path::new(&file_path_str);
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();

                    match get_date_created_from_file_exif(&file_path_str) {
                        Ok(date_created) => {
                            match date_created {
                                Some(file_datetime) => {
                                    info!("date created: {}", file_datetime);

                                    reorganize_file(dest_path, &file_path_str, file_datetime, file_name);
                                }
                                None => {
                                    warn!("file '{}' doesn't contain date in EXIF meta-data", file_name);

                                    if extract_dates_from_path {
                                        let extracted_dates = get_dates_from_path(&file_path_str);

                                        if !extracted_dates.is_empty() {
                                            let file_date = extracted_dates.last().unwrap();
                                            let result_date_format = file_date.format("%Y-%m-%d");
                                            let result_filename = format!("{}__{}", result_date_format, file_name);
                                            info!("result filename: '{}'", result_filename);
                                            create_year_dir_if_not_exists(dest_path, file_date.year())?;

                                            let month_name = get_month_name(file_date.month());

                                            let result_path = format!("{}/{}/{}", dest_path, file_date.year(), month_name);
                                            info!("result_path: '{}'", result_path);

                                            match fs::create_dir_all(&result_path) {
                                                Ok(_) => {
                                                    let result_file_path = format!("{}/{}", &result_path, result_filename);

                                                    info!("result file path '{}'", result_file_path);

                                                    info!("copy '{}' > '{}'", file_path_str, result_file_path);

                                                    match fs::copy(&file_path_str, &result_file_path) {
                                                        Ok(_) => {
                                                            info!("file has been copied");
                                                        }
                                                        Err(e) => {
                                                            error!("unable to copy file to destination: {}", e);
                                                        }
                                                    }
                                                }
                                                Err(e) => error!("unable to create path '{}': {}", result_path, e)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            warn!("file '{}' doesn't contain EXIF meta-data", file_name);

                            if extract_dates_from_path {
                                let extracted_dates = get_dates_from_path(&file_path_str);

                                if !extracted_dates.is_empty() {
                                    let file_date = extracted_dates.last().unwrap();
                                    let result_date_format = file_date.format("%Y-%m-%d");
                                    let result_filename = format!("{}__{}", result_date_format, file_name);
                                    info!("result filename: '{}'", result_filename);
                                    create_year_dir_if_not_exists(dest_path, file_date.year())?;

                                    let month_name = get_month_name(file_date.month());

                                    let result_path = format!("{}/{}/{}", dest_path, file_date.year(), month_name);
                                    info!("result_path: '{}'", result_path);

                                    match fs::create_dir_all(&result_path) {
                                        Ok(_) => {
                                            let result_file_path = format!("{}/{}", &result_path, result_filename);

                                            info!("result file path '{}'", result_file_path);

                                            info!("copy '{}' > '{}'", file_path_str, result_file_path);

                                            match fs::copy(&file_path_str, &result_file_path) {
                                                Ok(_) => {
                                                    info!("file has been copied");
                                                }
                                                Err(e) => {
                                                    error!("unable to copy file to destination: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => error!("unable to create path '{}': {}", result_path, e)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }

        Ok(())
    }

    fn get_files(path: &str) -> Result<Vec<String>, io::Error> {
        let mut results: Vec<String> = Vec::new();

        let dir_path = Path::new(path);

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            debug!("file: {:?}", entry.path().file_name());

            let file_type = entry.file_type()?;

            if file_type.is_file() || file_type.is_symlink() {
                let file_name = entry.file_name().into_string().unwrap();

                debug!("filename: {}", file_name);

                results.push(String::from(entry.path().to_str().unwrap()));
            }

            if file_type.is_dir() {
                if let Ok(files) = get_files(entry.path().to_str().unwrap()) {
                    files.iter().for_each(|file_path| results.push(String::from(file_path)));
                }
            }
        }

        Ok(results)
    }

    fn create_year_dir_if_not_exists(output_path: &str, year: i32) -> Result<(), io::Error> {
        let dir_name = format!("{}/{}", output_path, year);
        let target_path = Path::new(&dir_name);

        if !target_path.exists() {
            fs::create_dir_all(target_path)?
        }

        Ok(())
    }

    fn get_date_created_from_file_exif(file_path: &str) -> Result<Option<NaiveDateTime>, ExifError> {
        info!("get exif 'date created' property from '{}'", file_path);

        let mut result: Option<NaiveDateTime> = None;

        match rexif::parse_file(&file_path) {
            Ok(exif) => {
                for entry in &exif.entries {
                    if entry.tag == ExifTag::DateTimeOriginal {
                        debug!("created date: {}", &entry.value_more_readable);

                        let file_datetime = NaiveDateTime::parse_from_str(
                            &entry.value_more_readable, "%Y:%m:%d %H:%M:%S"
                        ).unwrap();

                        result = Some(file_datetime.to_owned());
                        break;
                    }
                }
                Ok(result)
            },
            Err(e) => {
                error!("unable to extract exif properties from '{}': {}", file_path, e);
                Err(e)
            }
        }
    }

    fn reorganize_file(dest_path: &str, file_path: &str,
                       file_datetime: NaiveDateTime, file_name: &str) -> Result<(), io::Error> {
        info!("date created: {}", file_datetime);

        let result_datetime_format = file_datetime.format("%Y-%m-%d__%H-%M-%S");

        let result_filename = format!("{}__{}", result_datetime_format, file_name);

        info!("result filename: '{}'", result_filename);

        create_year_dir_if_not_exists(dest_path, file_datetime.year())?;

        let month_name = get_month_name(file_datetime.month());

        let result_path = format!("{}/{}/{}", dest_path, file_datetime.year(), month_name);
        info!("result_path: '{}'", result_path);

        match fs::create_dir_all(&result_path) {
            Ok(_) => {
                let result_file_path = format!("{}/{}", &result_path, result_filename);

                info!("result file path '{}'", result_file_path);

                info!("copy '{}' > '{}'", &file_path, result_file_path);

                match fs::copy(&file_path, &result_file_path) {
                    Ok(_) => {
                        info!("file has been copied");
                        Ok(())
                    }
                    Err(e) => {
                        error!("unable to copy file to destination: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("unable to create path '{}': {}", result_path, e);
                Err(e)
            }
        }
    }

    fn get_month_name(month_index: u32) -> String {
        match month_index {
            1 => String::from("Январь"),
            2 => String::from("Февраль"),
            3 => String::from("Март"),
            4 => String::from("Апрель"),
            5 => String::from("Май"),
            6 => String::from("Июнь"),
            7 => String::from("Июль"),
            8 => String::from("Августа"),
            9 => String::from("Сентябрь"),
            10 => String::from("Октябрь"),
            11 => String::from("Ноябрь"),
            12 => String::from("Декабрь"),
            _ => String::from("Неизвестный")
        }
    }
}
