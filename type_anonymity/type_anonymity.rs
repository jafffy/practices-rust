#![allow(unused_variables)]
fn apply<F>(f: F) where
F: FnOnce() {
    f();
}

fn main() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}
