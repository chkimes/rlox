use std::vec::Vec;
use Value::*;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl Value {
    pub fn is_bool(&self) -> bool {
        match self {
            Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Number(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Bool(b) => b,
            _       => panic!("Value not bool"),
        }
    }

    pub fn as_number(self) -> f64 {
        match self {
            Number(n) => n,
            _       => panic!("Value not number"),
        }
    }
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
}

pub fn print_value(value: Value) {
    match value {
        Bool(b) => print!("{}", b),
        Number(n) => print!("{}", n),
        Nil => print!("nil"),
    }
}
