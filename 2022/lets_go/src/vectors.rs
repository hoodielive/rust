pub fn run()
{
    // Vectors are resizeable arrays.
    // You will more than likely prefer vectors over arrays.
    let mut numbers: Vec<i32> = vec![1, 2, 3];

    println!("Vector Length: {:?}", numbers);

    // Add onto Vector.
    numbers.push(5);

    println!("Vector Length: {:?}", numbers);

    // Pop off last value.
    numbers.pop();

    println!("Vector Length: {:?}", numbers);

    // Loop through vector values.
    for x in numbers.iter()
    {
        println!("Number: {:?}", x);
    }


    // Loop & mutate.
    for x in numbers.iter_mut()
    {
        *x *= 2;
    }

    println!("Numbers vec mutated: {:?}", numbers)
}
