#![allow(dead_code)]

#[allow(unused_imports)]
use print::{print_array, print_bytes};

mod print;

fn main() {
    playground();
}

#[inline(never)]
pub fn playground() {
    let numbers = [0x68, 0x69, 0x0a, 0];

    print_bytes(&numbers);
}
