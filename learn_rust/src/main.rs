use std::mem; 

fn main() {
    let a:u8 = 123; // 8 bits [u=unsigned number 0 +] i = is signed -127 to 128
    println!("a = {}", a); 

    //a = 456; - will generate an error because (let)
    // mut = mutable 
    let mut b:i8 = 0; // mutable 
    println!("b = {}", b); 
    b = 42; 
    println!("b = {}", b); 

    let mut c = 123456789; // 32-bit signed i32 
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c)); 
    c = -1; 
    println!("c = {} after modification", c); 
    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // isize/usize 
    let size_of_z = mem::size_of_val(&z); 
    println!("z = {}, takes up {} bytes, {}-bit os",
        z, size_of_z, size_of_z * 8
    ); 

    let d = 'x'; 
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d)); 

    let e = 2.5; 
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e)); 

}
