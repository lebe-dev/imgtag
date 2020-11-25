pub mod files {
    use std::{io, fs};
    use std::path::Path;

    pub fn get_files_from_path(path: &str) -> Result<Vec<String>, io::Error> {
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
                if let Ok(files) = get_files_from_path(entry.path().to_str().unwrap()) {
                    files.iter().for_each(|file_path| results.push(String::from(file_path)));
                }
            }
        }

        Ok(results)
    }
}
