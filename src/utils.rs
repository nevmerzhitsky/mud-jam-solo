use std::cell::RefCell;
use std::rc::Rc;

pub fn none_or_panic<T>(o: Option<T>, msg: &str) {
    match o {
        Some(_) => panic!("{}", msg),
        None => (),
    }
}

pub trait BuildRef {
    fn build_ref(self) -> Rc<RefCell<Self>>;
}
