fn say_hello(name: String) {
    println!("Hello {name}");
}

fn main() {
    // the Clone traits enables type to make copy of themselves
    // .clone(), vec! Box::new are spots where heap allocation happens

    let name = String::from("Brian");
    say_hello(name.clone());
    say_hello(name.clone());
    say_hello(name.clone());

    // it is common to clone your way out of problems with the borrow checker and optimize later
    // clone usually does a deep copy the value
}
