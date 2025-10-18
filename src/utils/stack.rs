

pub struct Stack<T> {
    data: Vec<T>,
    current: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            current: 0,
        }
    }
    
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.current += 1;
    }
    
    pub fn peek(&self) -> Option<&T> {
        if self.current <= 0 {
            return None;
        }
        
        self.data.get(self.current - 1)
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.current <= 0 {
            return None;
        }
        
        self.current -= 1;
        self.data.pop()
    }
}