#![allow(dead_code)]

#[allow(unused_imports)]
use print::{print_array, print_bytes};

mod print;

fn main() {
    playground();
}

#[inline(never)]
pub fn playground() {
    let mut x = 42;
    x += 1;

    print_bytes(&x);
}
