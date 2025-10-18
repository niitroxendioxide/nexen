
use std::fs::{File};
use std::io::{self, Read, Write};

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

pub fn write_to_output(source_file: &str, contents: &str) -> io::Result<()> {
    let file_name_opt = source_file.split("/").last();
    let output_file = if let Some(file_name) = file_name_opt {
        let without_nx = file_name.strip_suffix(FILE_EXTENSION).unwrap_or(file_name);
        
        format!("{}.{}", without_nx, FILE_EXTENSION)
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"));
    };
    
    match File::create(output_file) {
        Ok(mut file) => {
            file.write_all(contents.as_bytes())?;
        },
        Err(e) => {
            println!("\x1b[31m[Error]:\x1b[0m Error when writing to output file");
            return Err(e);
        }
    }

    Ok(())
}
