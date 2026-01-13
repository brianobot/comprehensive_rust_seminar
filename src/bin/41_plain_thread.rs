use std::thread;
use std::time::Duration;


/// Rust Threads work similar to thread in other languages
/// 
/// Spawn threads do not delay the main thread, if the main thread finishes before them, they are kaboom
/// Spawned threads can fail independently without affecting other threads
/// if you want to wait for a thread to finish before ending the main thread, use join methof on the thread handle
/// let handle = thread::spawn(|| {});
/// handle.join() // this would block the current thread, until the thread being handled is down

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..=10 {
            println!("Count in thread: {i}");
            thread::sleep(Duration::from_millis(5));
        }
        9
    });
    
    let value = handle.join().unwrap();
    
    // for i in 0..5 {
    //     println!("Main thread: {i}");
    //     thread::sleep(Duration::from_millis(5));
    // }
    println!("About to Go Into Sleep...");
    thread::sleep(Duration::from_secs(2));
    println!("Main thread is about to Die");
    
    
}