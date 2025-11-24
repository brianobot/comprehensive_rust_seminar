use std::io::Write;

fn main() {
    // A trait represent the a functionality or capabilty a type can support or do
    // for example a type that implement the Write trait can write bytes out
    // a type that implements the Iterator trait can produce a sequence of values
    // a type that implements the clone trait can make clones of itself
    // 
    let mut some_data: Vec<u8> = vec![1, 2, 3, 4, 5];
    
    some_data.write(&[30u8]).unwrap();
    println!("Some data = {:?}", some_data);
    // the Trait itself must be in scope otherwise it's method can not be called on a type
    // the reason some Trait method work without importing the Trait is because there are already in scope
    // due to the prelude which makes some commonly used type available in all rust program by default
}