
use std::fs::File;
use std::io::{self, Read};

use crate::files::FILE_EXTENSION;

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn validate_and_read_file(file_path: &str) -> Result<String, String> {
    let is_correct_extension = file_path.ends_with(FILE_EXTENSION);
    if !is_correct_extension {
        return Err(format!("Invalid file extension. Expected .nx, got .{}", file_path.split(".").last().unwrap()));
    }

    let contents = read_file(file_path);
    match contents {
        Ok(contents) => Ok(contents),
        Err(err) => Err(format!("Error reading file: {}", err)),
    }
}