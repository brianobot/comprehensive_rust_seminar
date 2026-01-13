use std::thread;
use std::time::Duration;


fn main() {
    // normal threads can not borrow from the environment
    let s = String::from("Brian");
    
    thread::spawn(move || {
        println!("Length of S = {}", s.len());
    });
    
    println!("This is the Main Thread");
    // thread::sleep(Duration::from_secs(1));
    // 
    // However you can use a scoped thread to borrow values from it's environment
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("THis is inside a scoped thread: ");
        });
    });
    
    
}