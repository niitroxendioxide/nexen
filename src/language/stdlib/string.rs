use crate::language::{datatypes::DataType, errors::LangError};

pub fn str_len(args: &[DataType]) -> Result<DataType, LangError> {
    if args.len() != 1 {
        return Err(LangError::new("Invalid number of arguments for 'str_len'".to_string()));
    }
    
    match args.get(0).unwrap() {
        DataType::String(val) => Ok(DataType::Float(val.len() as f32)),
        _ => Err(LangError::new("Invalid argument for 'str_len'".to_string())),
    }
}

pub fn str_to_num(args: &[DataType]) -> Result<DataType, LangError> {
    if args.len() != 1 {
        return Err(LangError::new("Invalid number of arguments for 'str_len'".to_string()));
    }

    match args.get(0) {
        Some(opt) => {
            return Ok(DataType::Float(opt.as_float()));
        },
        None => {
            return Ok(DataType::EndOfBlock);
        }
    }
}