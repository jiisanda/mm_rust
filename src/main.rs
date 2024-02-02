#![allow(dead_code)]

#[allow(unused_imports)]
use print::{print_array, print_bytes};

mod print;
mod lor;
mod happy_lor;
mod lor_rc;

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone)]
struct PointCopy {
    x: i32,
    y: i32,
}

fn main() {
    lor_rc::share_the_ring();
    // playground();
}

#[inline(never)]
pub fn playground() {
    let mut point = PointCopy { x: 15, y: 14 };
    point.x += 1;
    point.y += 2;

    let mut point2 = point;
    point2.x += 1;
    point2.y += 2;

    print_bytes(&point);
    print_bytes(&point2);
}
