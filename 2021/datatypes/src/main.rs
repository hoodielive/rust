use std::mem; 

const MEANING_OF_LIFE:u8 = 42;  // no fixed address 
static Z:i32 = 123; 

fn main() {
    // primitives 
    let a:u8 = 123; // 8 bits - unsigned integer is positive or 0..255 if signed -255..0 
    println!("a = {}", a); 

    // a = 456; would generate an error without let 'mut' 
    let mut b:i8 = 0; // mutable

    println!("b = {}", b); 

    b = 42;

    println!("b = {}", b); 

    let mut c = 123456789; // rust can guess what type of value this is: 32-bit signed i32 - takes up 4 bytes because sval 4bit

    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c)); 
    c = -1; 
    println!("c = {}, size = {} after modification", c, mem::size_of_val(&c)); 
    // i8 u8 i16 u16 i32 u32 i64 u64

    let z:isize = 123; // isize/usize 
    let size_of_z = mem::size_of_val(&z); 
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8); 

    let d:char = 'x'; 
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d)); 

    let e = 2.5;  // double-precision, 8 bytes or 64 bits, f64 or express it as let e:f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e)); 

    // boolean true false 
    let g = false; 
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g)); 
    let f = 4>0; // true

    println!("The meaning of life is: {} ", MEANING_OF_LIFE); 
    unsafe {
        println!("I want a fixed address: {}", Z); 
    }
}

fn operators() {
    // arithmetic 
    let mut a = 2+3*4; 
    
    a = a + 1; // -- ++ not supported in rust, it does however support -= += /= *= %= 
    println!("remainder of {} / {} = {}", a, 3, (a%3)); // there is no power operator either :< , but you can: 
    let a_cubed = i32::pow(a, 3); 
    println!("{} cubed is {}", a, a_cubed); 

    let b = 2.5; 
    let b_cubed = f64::powi(b, 3); // b x b x b => powi 
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // this is how to you take the constants 
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi); 

    // bitwise 
    let c = 1 | 2;  // | OR & AND ^ XOR ! NOR -> 01 OR 10 = 11 == 3_10 
    println!("1|2 = {}", c); 
    let two_to_10 = 1 << 10; // shift this 10 spaces to the left; 
    println!("2^10 = {}", two_to_10); 

    // logical 
    let pi_less_4 = std::f64::consts::PI < 4.0; // true 
    let x = 5;  // > >= <= < == 
    let x_is_5 = x == 5; // true
}

fn scope_and_shadowing() {
    let a = 123; 
    {
        let b = 456; 
        println!("inside, b = {}", b); 
    }
    println!("a = {}", a); 
}

fn scopes() {
    // scope is created by {}
    scope_and_shadowing();
}
