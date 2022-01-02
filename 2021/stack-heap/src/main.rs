#![allow(dead_code)]
#![allow(unused_variable)]

mod sh;
use std::mem; 

fn main() {
    // both stack and heap deal with memory allocation 
    // stack  (let x = )is used more for temporary memory allocation whereas: 
    // heap ( let x = Box::new(5); )is a chunk of memory for long term storage 

    sh::stack_and_heap(); 
}
