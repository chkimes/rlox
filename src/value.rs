use crate::object::*;
use std::vec::Vec;
use Value::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Object(Obj),
    Nil
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
}

pub fn print_value(value: &Value) {
    match value {
        Bool(b) => print!("{}", b),
        Number(n) => print!("{}", n),
        Object(o) => o.print_obj(),
        Nil => print!("nil"),
    }
}
