use tokio::time;

// Tokio provides:

// A multi-threaded runtime for executing asynchronous code.
// An asynchronous version of the standard library.
// A large ecosystem of libraries.

async fn count_to(count: i32) {
    for i in 0..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}


#[tokio::main]
async fn main() {
    let handle = tokio::spawn(count_to(10));
    handle.await.unwrap();
    
    // count_to(10).await;
    // for i in 0..5 {
    //     println!("Main task: {i}");
    //     time::sleep(time::Duration::from_millis(5)).await;
    // }
}