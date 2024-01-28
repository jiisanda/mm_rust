#![allow(dead_code)]

#[allow(unused_imports)]
use print::{print_array, print_bytes};

mod print;

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
    playground();
}

#[inline(never)]
pub fn playground() {
    let mut point = Box::new(Point { x: 15, y: 14 });
    point.x += 1;
    point.y += 2;

    print_bytes(&point);
}
