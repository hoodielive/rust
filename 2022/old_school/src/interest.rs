pub fn run()
{
    let named_var = "obatala";
    println!("Give me some help baby: {}", named_var);


    for x in named_var.split_whitespace()
    {
        println!("{} am Great!", x);
    }
}
