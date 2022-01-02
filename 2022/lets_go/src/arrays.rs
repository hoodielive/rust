use std::mem;

pub fn run()
{
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Debug.
    println!("{:?}", numbers);

    // Get single value.
    println!("Single value: {}", numbers[0]);

    // Reassign value.
    numbers[2] = 30;

    // Get single value.
    println!("Single value: {}", numbers[0]);

    // Get length.
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes.", std::mem::size_of_val(&numbers));

    // use std::mem;
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers;

    println!("Slice: {:?}", slice);

    let _slice_2: &[i32] = &numbers[0..2];

}
