// bindings (variables) in rust are immutable

fn main() {
    say_hello("Rusty");
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name)
}

let var = "value";

