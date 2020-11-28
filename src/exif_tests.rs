#[cfg(test)]
pub mod exif_tests {
    use crate::exif::exif::get_date_created_from_file_exif;

    #[test]
    fn return_error_for_unsupported_date_format() {
        match get_date_created_from_file_exif("img-src/wrong-exif/wrong-exif.jpg") {
            Ok(value) => assert!(value.is_none()),
            Err(_) => panic!("result expected")
        }
    }
}
