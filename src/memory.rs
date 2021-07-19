use std::any::Any;

pub struct Heap {
    values: Vec<Box<dyn Any>>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap { values: Vec::new() }
    }

    pub fn manage<T: 'static>(&mut self, obj: T) -> Ref<T> {
        let boxed = Box::new(obj);
        let r = Ref { obj: &*boxed };
        self.values.push(boxed);
        r
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