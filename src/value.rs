use crate::object::*;
use Value::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Object(Obj),
    Nil
}

impl Value {
    pub fn print(&self) {
        match self {
            Bool(b) => print!("{}", b),
            Number(n) => print!("{}", n),
            Object(o) => o.print_obj(),
            Nil => print!("nil"),
        }
    }
}
