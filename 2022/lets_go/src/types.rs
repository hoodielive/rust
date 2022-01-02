pub fn run()
{
    // Default i32
    let x: i32 = 1;

    // Default is f64
    let y: f32 = 2.5f32; 

    // Add explicit type
    let z: i64 = 454544545545;

    // Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Bool
    let is_active: bool = true;

    // Get Boolean from expression
    let _is_greater: bool = 10 > 5;

    // Char
    let _a1: char = 'a';
    let _face = '\u{1F600}'; // emoji unicode for smiley face.

    println!("{:?}", (x, y, z, is_active, _face));
}
