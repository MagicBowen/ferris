use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(1));
    let n = value.borrow();
    println!("{}", *n);
    drop(n);
    drop(value);
}
