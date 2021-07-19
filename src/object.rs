use crate::memory::*;
use crate::object::Obj::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Obj {
    LString(Ref<String>),
}

impl Obj {
    pub fn print_obj(&self) {
        match self {
            LString(s) => print!("\"{}\"", s.obj())
        }
    }
}