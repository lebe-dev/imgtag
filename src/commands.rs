pub mod commands {
    use std::{io, fs};
    use rexif::{ExifError, ExifTag};
    use std::path::Path;
    use chrono::{NaiveDateTime, Datelike, NaiveDate};
    use crate::path_parser::path_parser::get_dates_from_path;
    use std::io::{Error, ErrorKind};

    pub fn reorganize_files(src_path: &str, dest_path: &str,
                            extract_dates_from_path: bool,
                            on_progress: fn(total: usize, current_index: usize))
                                                                        -> Result<(), io::Error> {
        info!("reorganize files for path '{}'", src_path);
        info!("destination path '{}'", dest_path);
        info!("extract dates from path: {}", extract_dates_from_path);

        let mut has_errors = false;

        match get_files(src_path) {
            Ok(files) => {
                for (index, file_path_str) in files.iter().enumerate() {
                    info!("processing file '{}'", file_path_str);

                    let file_path = Path::new(&file_path_str);
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();

                    match get_date_created_from_file_exif(&file_path_str) {
                        Ok(date_created) => {
                            match date_created {
                                Some(file_datetime) => {
                                    let (result_path, result_file_path) =
                                        get_dest_path_and_filepath_with_datetime(
                                        dest_path, file_name,
                                        file_datetime
                                    );

                                    match reorganize_file(
                                        file_datetime.year(), dest_path,
                                        &file_path_str, &result_path,
                                        &result_file_path
                                    ) {
                                        Ok(_) => {}
                                        Err(_) => has_errors = true
                                    }
                                }
                                None => {
                                    warn!(
                                        "file '{}' doesn't contain date in EXIF meta-data",
                                        file_name
                                    );

                                    if extract_dates_from_path {
                                        match reorganize_file_without_exif(
                                            &file_path_str, dest_path, file_name
                                        ) {
                                            Ok(_) => {}
                                            Err(_) => has_errors = true
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            warn!("file '{}' doesn't contain EXIF meta-data", file_name);

                            if extract_dates_from_path {
                                match reorganize_file_without_exif(&file_path_str, dest_path,
                                                                   file_name) {
                                    Ok(_) => {}
                                    Err(_) => has_errors = true
                                }
                            }
                        }
                    }

                    on_progress(files.len(), index)
                }

                if !has_errors {
                    Ok(())

                } else {
                    Err(Error::from(ErrorKind::Other))
                }
            }
            Err(e) => {
                error!("unable to get files: {}", e);
                Err(e)
            }
        }
    }

    fn reorganize_file_without_exif(file_path_str: &str,
                                    dest_path: &str, file_name: &str) -> Result<(), io::Error> {
        let extracted_dates = get_dates_from_path(&file_path_str);

        if !extracted_dates.is_empty() {
            let file_date = extracted_dates.last().unwrap();
            let (result_path, result_file_path) = get_dest_path_and_filepath_with_date(
                dest_path, file_name, file_date
            );

            reorganize_file(file_date.year(), dest_path,
                            &file_path_str, &result_path,
                            &result_file_path)

        } else {
            info!("unable to reorganize file because file path doesn't \
                   contain any information about date");
            Ok(())
        }
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

    fn get_date_created_from_file_exif(file_path: &str) ->
                                                        Result<Option<NaiveDateTime>, ExifError> {
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

    fn get_dest_path_and_filepath_with_datetime(root_dest_path: &str, original_file_name: &str,
                                                file_datetime: NaiveDateTime) -> (String, String) {
        let result_datetime_format = file_datetime.format("%Y-%m-%d__%H-%M-%S");

        let result_filename = format!("{}__{}", result_datetime_format, original_file_name);

        info!("result filename: '{}'", result_filename);

        let month_name = get_month_name(file_datetime.month());

        let result_path = format!("{}/{}/{}", root_dest_path, file_datetime.year(), month_name);
        info!("result_path: '{}'", result_path);

        let result_file_path = format!("{}/{}", &result_path, result_filename);

        (result_path, result_file_path)
    }

    fn get_dest_path_and_filepath_with_date(root_dest_path: &str, original_file_name: &str,
                                            file_datetime: &NaiveDate) -> (String, String) {
        let result_datetime_format = file_datetime.format("%Y-%m-%d");

        let result_filename = format!("{}__{}", result_datetime_format, original_file_name);

        info!("result filename: '{}'", result_filename);

        let month_name = get_month_name(file_datetime.month());

        let result_path = format!("{}/{}/{}", root_dest_path, file_datetime.year(), month_name);
        info!("result_path: '{}'", result_path);

        let result_file_path = format!("{}/{}", &result_path, result_filename);

        (result_path, result_file_path)
    }

    fn reorganize_file(year: i32, root_dest_path: &str, src_file_path: &str,
                       dest_path: &str, dest_file_path: &str) -> Result<(), io::Error> {
        create_year_dir_if_not_exists(root_dest_path, year)?;

        match fs::create_dir_all(&dest_path) {
            Ok(_) => {
                info!("copy '{}' > '{}'", &src_file_path, &dest_file_path);

                match fs::copy(&src_file_path, &dest_file_path) {
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
                error!("unable to create path '{}': {}", &dest_path, e);
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
