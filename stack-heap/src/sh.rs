#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64, 
    y: f64
}
fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}
pub fn stack_and_heap() {
    let p1 = origin();  // pl:Point (stack allocated)
    let p2 = Box::new(origin()); // p2:Point (heap allocated)
    
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); 
}
