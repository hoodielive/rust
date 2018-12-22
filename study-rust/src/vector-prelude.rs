fn main() {
    let mut v = Vec::new(); 
// in this you don't have to use std::collections::Vec above; because it's included automatically
    v.push('x'); 
    v.push('y');
    v.push('z');

    for item in v {
        println!("{}", item);
    }
}
