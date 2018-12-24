fn main() {
    // both stack and heap deal with memory allocation 
    // stack  (let x = )is used more for temporary memory allocation whereas: 
    // heap ( let x = Box::new(5); )is a chunk of memory for long term storage 

    let x = 5; // i32
    let y = 32; // i32 
    f(x,y)
}
