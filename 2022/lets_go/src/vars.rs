pub fn run()
{
    // Rust is a blocked-scope language.

    // Variables.
    let name = "Broski";

    println!("My name is: {}", name);

    // You need mut (mutate) in order to reassign to this variable.
    let mut _age = 30;

    println!("{}, is {} years old.", name, _age);

    _age = 31;

    println!("My bad, he is {} years old.", _age);

    // Define a constant.
    const ID: i32 = 001; // Struct.
    println!("ID: {}", ID);

    // Assgin multiple vars.
    let ( _my_name, _my_age ) = (name, _age);
}
