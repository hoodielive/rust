#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,  // 16 bits
    y: f64 // 8 bytes
}
fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}
pub fn stack_and_heap() {
    let p1 = origin();  // pl:Point (stack allocated)
    let p2 = Box::new(origin()); // p2:Point (heap allocated)
    
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); 
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); 

    let p3 = *p2;  // tell rust to follow the pointer (where the box value lives)
    println!("p3 takes up {} bytes", p3.x);  // realloc back to stack 
}
