pub mod domain {
    /// Behaviour config for files without EXIF or without 'Date created' exif-property.
    pub struct NoExifConfig {
        pub extract_dates_from_path: bool,
        pub force_year: bool,
        pub year: i32
    }

    impl NoExifConfig {
        pub fn to_string(&self) -> String {
            return String::from(
                format!(
                    "extract_dates_from_path: {}, force_year: {}, year: {}",
                    self.extract_dates_from_path, self.force_year, self.year
                )
            );
        }
    }
}
