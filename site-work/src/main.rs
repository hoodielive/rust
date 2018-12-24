fn work_with_slice() {
    let array: [i32; 5] = [0,1,2,3,4]; 
    let slice = &array[0..3]; 

    for x in slice {
        println!{"x is slice {}", x}; 
    }
}

fn main() {
    let condition = true; 
    let _number = if condition {
        5 
    } else {
        4 
    };
    println!("The number is {}", _number)
work_with_slice()
}


