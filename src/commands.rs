pub mod commands {
    use std::{io, fs};
    use rexif::{ExifError, ExifTag};
    use std::path::Path;
    use chrono::{NaiveDateTime, Datelike, NaiveDate, Local, TimeZone};
    use crate::path_parser::path_parser::get_dates_from_path;
    use std::io::{Error, ErrorKind};
    use crate::domain::domain::NoExifConfig;
    use crate::files::files::get_files_from_path;
    use crate::exif::exif::get_date_created_from_file_exif;

    pub fn reorganize_files(src_path: &str, dest_path: &str,
                            no_exif_config: &NoExifConfig,
                            on_progress: fn(total: usize, current_index: usize))
                                                                        -> Result<(), io::Error> {
        info!("reorganize files for path '{}'", src_path);
        info!("destination path '{}'", dest_path);
        info!("no exif config: {}", no_exif_config.to_string());

        let mut has_errors = false;

        match get_files_from_path(src_path) {
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

                                    if no_exif_config.extract_dates_from_path {
                                        match reorganize_file_without_exif(
                                            &file_path_str, dest_path, file_name,
                                            no_exif_config
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

                            if no_exif_config.extract_dates_from_path {
                                match reorganize_file_without_exif(
                                    &file_path_str, dest_path,
                                    file_name, no_exif_config
                                ) {
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
                                    dest_path: &str, file_name: &str,
                                    no_exif_config: &NoExifConfig) -> Result<(), io::Error> {

        if no_exif_config.force_year {
            let local_dt = Local.ymd(no_exif_config.year, 1, 1)
                .and_hms_milli(9, 10, 11, 12);
            let file_date = local_dt.naive_local().date();

            let (result_path, result_file_path) = get_dest_path_and_filepath_with_date(
                dest_path, file_name, &file_date
            );

            reorganize_file(file_date.year(), dest_path,
                            &file_path_str, &result_path,
                            &result_file_path)

        } else {
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
    }

    fn create_year_dir_if_not_exists(output_path: &str, year: i32) -> Result<(), io::Error> {
        let dir_name = format!("{}/{}", output_path, year);
        let target_path = Path::new(&dir_name);

        if !target_path.exists() {
            fs::create_dir_all(target_path)?
        }

        Ok(())
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
            8 => String::from("Август"),
            9 => String::from("Сентябрь"),
            10 => String::from("Октябрь"),
            11 => String::from("Ноябрь"),
            12 => String::from("Декабрь"),
            _ => String::from("Неизвестный")
        }
    }
}
