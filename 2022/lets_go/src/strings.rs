
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

    // Is string empty?
    println!("Is empty?: {}", _hello.is_empty());

    // Contains?
    println!("Contains 'World': {}", _hello.contains("World"));

    // Replace.
    println!("Replace : {}", _hello.replace("World", "There"));

    // Loop through string by whitespace.
    for word in _hello.split_whitespace()
    {
        println!("{}",word);
    }

    // Create string with capacity.
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing.
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s)
}
