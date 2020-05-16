#[derive(Debug)]
struct Position {
    x: i8,
    y: i8,
}


fn main() {
    let mut pos: Position = Position {x: 42, y: 32};
    pos.x = 13;
    let pos2: &mut Position = &mut pos;
    //pos2.x = 12;
    println!("My Position is: {:#?}", pos);
   //println!("My Position is: {:#?}", pos2);
}
