pub fn run()
{
    let age: u8 = 18;
    let _check_id: bool = false;
    let _knows_person_of_age: bool = true;

    if age >= 21 && _check_id
    {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && _check_id || _knows_person_of_age {
        println!("You are not old enough to drink.");
    }
    else {
        println!("I'll need to see your ID.");
    }

}
