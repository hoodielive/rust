

fn main() {
    let greeting = "Howdy";
    let subject = "Aeon";
    let formated_math =format!("{:04}", 42);
    let formatted_principles = format!("{}, {}!", greeting, subject);

    println!("Hello {formatted_principles}");
    format!("hello");
    format!("hello, {}!", 1);
    format!("The number is {}", 1);
    format!("{:?}", (3, 4));
    println!("This is an underwhelming view of an {formated_math}")
}
