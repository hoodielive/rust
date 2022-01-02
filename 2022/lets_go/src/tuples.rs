pub fn run()
{
    // Tuples group together values of different types.
    // MAX::12 elements.

    let _person: (&str, &str, i8) =
        (
            "Broski",
            "Osa",
            30
        );

    // Debug.
    println!("{} is from {} and is {}", _person.0, _person.1, _person.2);
}
