
pub fn run()
{
    // &str
    let mut _hello = String::from("Hello");

    // Debug.
    println!("{}", _hello);
    println!("{}", _hello.len());

    // Push char.
    _hello.push('w');

    // Debug.
    println!("{}", _hello);

    // Push string.
    _hello.push_str("orld");

    // Debug.
    println!("{}", _hello);

    // Examine Capacity.
    println!("Capacity: {}", _hello.capacity());

    // Debug.
    println!("{}", _hello);

}
