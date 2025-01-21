use std::cell::RefCell;

fn main() {
    let mut rc = RefCell::new(String::from("hello"));

    let mut s = rc.get_mut();
    println!("{}", s);
}