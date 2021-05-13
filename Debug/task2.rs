#[derive(Debug)]
struct Person<'s> {
    name: &'s str,
    age : u8
}
fn main()
{
    let name = "srinath";
    let age = 19;
    let sri = Person{name, age};

    println!("{:#?}",sri);
}