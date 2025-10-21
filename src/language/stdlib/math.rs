use std::{ops::{Add, Mul}, time::{SystemTime, UNIX_EPOCH}};

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

pub fn random(_: &[DataType]) -> Result<DataType, LangError> {
    let rand_seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let next_u64 = rand_seed.mul(8934589234578902345 as u64).add(39485904390230459 as u64);
    let float_res = next_u64 as f32 / u64::MAX as f32;
    
    Ok(DataType::Float(float_res))
}
