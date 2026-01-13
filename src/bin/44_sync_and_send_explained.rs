use std::thread;


#[allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    id: String,
    
}

fn main() {
    // Send and Sync are traits 
    // they are marker traits in that they do not have any function implementations and
    // are only there to mark the status of their implementors
    // they are used at compile time to ensure certain standards are met
    // Some other examples of marker traits are Sized, Unpin
    // Auto traits are traits that are implemented for you automatically if all your internals types implement them
    // 
    // So Send and Sync are auto market traits that allow the compiler to enforce certain rules at compile time
    // 
    // Send: if a type is allowed to be sent into a Thread
    // Sync: if &T can be sent and shared between threads
    // 
    println!("Let's get Started");
    
    let _value = String::from("value");
    let my_struct = MyStruct{ id: String::from("0001") };
    
    thread::scope(|scope| {
        scope.spawn(|| {
            dbg!(&my_struct) // the fact that we can move a value into a thread indicates that the type implemeents the Send trait
            // it's important to note that types that implement COpy would be copied when a move is used like this
            // and in that case the code would stil compile even if that type doesn't implement a Send
        });
        
        // the fact that we can reference a value in multiple trait without the compiler crying shows that the type implements the Sync trait
        scope.spawn(|| {
            dbg!(&my_struct)
        });
    });
    
    
    let second_value = std::sync::Mutex::new(0);
    
    thread::scope(|scope| {
        scope.spawn(|| {
            dbg!(&second_value);
        });
        
        scope.spawn(|| {
            dbg!(&second_value);
        });
    });
}