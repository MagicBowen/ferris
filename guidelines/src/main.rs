use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(1));
    let n = value.borrow();
    println!("{}", *n);
    drop(n);
    drop(value);
}
