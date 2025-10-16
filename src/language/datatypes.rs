#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Float(f32),
    Bool(bool),
    String(String),
    EndOfBlock,
}

#[derive(PartialEq)]
pub enum DataTypeType {
    Float,
    Bool,
    String,
}

impl DataType {
    pub fn as_float(&self) -> f32 {
        match self {
            DataType::Float(val) => *val,
            DataType::Bool(b) => if *b { 1.0 } else { 0.0 },
            DataType::String(str) => str.parse::<f32>().unwrap_or(0.0),
            DataType::EndOfBlock => panic!("Cannot evaluate eof as bool"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            DataType::Bool(b) => *b,
            DataType::Float(val) => *val != 0.0,
            DataType::String(str) => {
                if str == &"true" {
                    return true;
                }

                false
            },
            DataType::EndOfBlock => panic!("Cannot evaluate eof as bool"),
        }
    }

    pub fn get_type(&self) -> DataTypeType {
        match self {
            DataType::Float(_) => DataTypeType::Float,
            DataType::Bool(_) => DataTypeType::Bool,
            DataType::String(_) => DataTypeType::String,
            DataType::EndOfBlock => panic!("Cannot get type of eof"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            DataType::String(str) => format!("{}", str),
            DataType::Float(val) => format!("{}", val),
            DataType::Bool(b) => format!("{}", b),
            DataType::EndOfBlock => panic!("Cannot evaluate eof as bool"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        self.as_bool()
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Float(val) => write!(f, "{}", val),
            DataType::Bool(b) => write!(f, "{}", b),
            DataType::String(str) => write!(f, "{}", str),
            DataType::EndOfBlock => Ok(()),
        }
    }
}