use std::collections::LinkedList; 

fn main() {
    // linked list is a struct
    let mut ll = LinkedList::new(); 
    
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(4);

    for item in ll {
        println!("{}", foo);
    }
}
