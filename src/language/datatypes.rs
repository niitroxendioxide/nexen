#[derive(Clone, Debug, PartialEq, Copy)]
pub enum DataType {
    Float(f32),
    Bool(bool),
}

impl DataType {
    pub fn as_float(&self) -> f32 {
        match self {
            DataType::Float(val) => *val,
            DataType::Bool(b) => if *b { 1.0 } else { 0.0 },
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            DataType::Bool(b) => *b,
            DataType::Float(val) => *val != 0.0,
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
        }
    }
}