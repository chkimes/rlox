use std::vec::Vec;

pub type Value = f64;

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
}

pub fn print_value(value: Value) {
    print!("{}", value);
}