use std::any::Any;

pub struct Heap {
    values: Vec<Box<dyn Any>>,
    strings: Vec<String>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            values: Vec::new(),
            strings: Vec::new(),
        }
    }

    pub fn manage<T: 'static>(&mut self, obj: T) -> Ref<T> {
        let boxed = Box::new(obj);
        let r = Ref { obj: &*boxed };
        self.values.push(boxed);
        r
    }

    pub fn manage_str(&mut self, str: String) -> Ref<String> {
        let item = self.strings.iter().find(|&s| *s == str);
        match item {
            Some(s) => Ref { obj: s },
            None    => {
                self.strings.push(str);
                Ref { obj: &self.strings[self.strings.len() - 1] }
            }
        }
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

pub struct Ref<T> {
    obj: *const T
}

impl<T> Copy for Ref<T> {}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Ref<T> {
        *self
    }
}

impl<T: PartialEq> PartialEq for Ref<T> {
    fn eq(&self, other: &Ref<T>) -> bool {
        self.obj().eq(other.obj())
    }
}

impl<T> Ref<T> {
    pub fn obj(&self) -> &T {
        unsafe { &*self.obj }
    }
}