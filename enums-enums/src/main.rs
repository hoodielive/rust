enum Color 
{
    Red, 
    Green,
    Blue, 
    RgbColor(u8,u8,u8)
}

fn enums()
{
//    let c:Color = Color::Red; 
    let d:Color = Color::RgbColor(10,0,0);
    match d
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        _ => println!("some other color")
    }
}
fn main() {
    enums(); 
}
