use std::fs;

pub fn read(file_name: String) -> String {
    return fs::read_to_string(file_name).unwrap()
}