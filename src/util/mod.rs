use std::fs;
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> String {
    let contents = fs::read_to_string(path).expect("Could not read file from path");
    return contents;
}
