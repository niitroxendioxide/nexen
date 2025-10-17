use crate::language::{datatypes::DataType, errors::LangError};

pub fn print(args: &[DataType]) -> Result<DataType, LangError> {
    if args.len() < 1 {
        return Err(LangError::new("Not enough arguments for 'println'".to_string()));
    }

    let iterator = args.iter();
    for arg in iterator {
        let mut second_par = "";
        if arg == &args[args.len() - 1] {
            second_par = "\n";
        }

        print!("{}{}", arg.as_string(), second_par);
    }

    Ok(DataType::EndOfBlock)
}
