pub fn run()
{
    // Print to Console.

    println!("Hello from the print.rs file.");
    println!("Number: {}", 1);

    println!(
            "{name} is having a {gender}!",
            name  = "Dosa",
            gender = "Girl"
    );

    println!("Binary: {:b}, Hex: {:x}, Octal: {:o} ", 10, 10, 10);

    // Placeholder for debug trait. Tuple.

    println!("{:?}", (12, true, "hello"));
}
