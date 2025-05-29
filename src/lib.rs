use std::cell::RefCell;
use std::rc::Rc;

pub trait BuildRef {
    fn build_ref(self) -> Rc<RefCell<Self>>;
}
