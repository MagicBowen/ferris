fn foo<F>(f: F) where F: Fn(&i32) {
    let z =0;
    f(&z)
}

fn main () {
    foo(|x| println!("{x}"));
}