use crate::language::{datatypes::DataType, errors::LangError};

fn fib_inner(p: f32) -> f32 {
    if p == 0.0 {
        0.0
    } else if p == 1.0 {
        1.0
    } else {
        fib_inner(p - 1.0) + fib_inner(p - 2.0)
    }
}

pub fn fib(args: &[DataType]) -> Result<DataType, LangError> {
    if args.len() != 1 {
        return Err(LangError::new(format!("Invalid number of arguments")));
    }

    match args[0] {
        DataType::Float(float) => {
            let result = fib_inner(float) as f32;
            Ok(DataType::Float(result))
        }
        _ => Err(LangError::new(format!("Cannot calculate Fibonacci number for non-numeric type")))
    }
}
