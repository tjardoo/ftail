use std::{fs::File, io::LineWriter, path::PathBuf, sync::Mutex};

use crate::Config;

pub(crate) fn rotate_if_exceeds_max_file_size(
    file: &Mutex<LineWriter<File>>,
    file_path: PathBuf,
    config: &Config,
) {
    if config.max_file_size.is_none() {
        return;
    }

    let mut file = file.lock().unwrap();

    let md = file.get_ref().metadata().unwrap();

    if md.len() > config.max_file_size.unwrap() {
        let path = file_path.to_str().unwrap();

        let mut new_path = format!("{}.old", path);

        let mut counter = 1;
        while std::fs::metadata(&new_path).is_ok() {
            new_path = format!("{}.old{}", path, counter);
            counter += 1;
        }

        std::fs::rename(path, &new_path).unwrap();

        let new_file = std::fs::File::create(path).unwrap();
        *file = LineWriter::new(new_file);
    }
}
