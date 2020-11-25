pub mod exif {
    use chrono::NaiveDateTime;
    use rexif::{ExifError, ExifTag};

    const DATETIME_FORMAT: &str = "%Y:%m:%d %H:%M:%S";

    pub fn get_date_created_from_file_exif(file_path: &str) ->
                                                        Result<Option<NaiveDateTime>, ExifError> {
        info!("get exif 'date created' property from '{}'", file_path);

        let mut result: Option<NaiveDateTime> = None;

        match rexif::parse_file(&file_path) {
            Ok(exif) => {
                for entry in &exif.entries {
                    if entry.tag == ExifTag::DateTimeOriginal {
                        debug!("created date: {}", &entry.value_more_readable);

                        match NaiveDateTime::parse_from_str(
                            &entry.value_more_readable, DATETIME_FORMAT
                        ) {
                            Ok(file_datetime) =>
                                result = Some(file_datetime.to_owned()),
                            Err(e) => error!("unsupported date format: '{}'", e)
                        }

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
}
