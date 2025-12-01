use crate::language::{datatypes::DataType, errors::LangError};

pub fn std_listen(_: &[DataType]) -> Result<DataType, LangError> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return Ok(DataType::String(input))
        },
        Err(_) => {
            return Ok(DataType::String("".to_string()))
        },
    }
}