fn main() {
    // Box is an owned pointer to data on the heap
    let five = Box::new(5);
    println!("Five: {}", *five);

    // Box<T> implements Deref<Target=T>, which means you can call methods of T directly on the Box
    // recursive data strcuture can not be stored inline on the stack but they can be stored with a pointer

    let name = Box::new(String::from("Brian"));
    println!("Name: {}", *name);

    // the two use cases for Box are
    // 1. Having a variable with a triat type that can not be computed at compile time
    // 2. Having a recursive data structure

    #[derive(Debug)]
    struct Truck {
        next_truck: Option<Box<Truck>>
    }

    let first_truck = Truck{ next_truck: Some(Box::new(Truck{ next_truck: None }))};
    dbg!(first_truck);

    // Box can not contain empty/null 
}