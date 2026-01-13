use futures::executor::block_on;
use rand::Rng;
use std::task::Poll;

/// This examples are illustrative, and aren’t an accurate representation of the Rust compiler’s transformation. 
/// The important things to notice here are:

// - Calling An Async Function does nothing but constructing and returning a Future
// - Struct fields, or Enums variants can be used to keep track of the future local progress
// - Unlike Python (Task) and JS(Promises) which are equivalent Future concepts, Futures are lazy, they do not start until they are polled

fn main() {
    // async is a concurrency model where multiple tasks are executed concurrently by executing each task
    // until it would block then switching to another task that is ready to make progress
    // 
    // Russt async operation is based on futures, which represent work that maybe completed in the future
    // any type that implements the the std::future::Future trait is a future.
    // 
    // futures re polled by an async runtime and several different runtimes are available
    block_on(async_main(10))
}

// async fn count_to(count: i32) {
//     for i in 0..count {
//         println!("Count is {i}");
//     }
// }

// fn count_to(count: i32) -> CountTo {
//     CountTo { count }
// }


#[allow(dead_code)]
fn count_to(count: i32) -> impl Future {
    CountTo { count }
}

struct CountTo {
    count: i32
}

impl Future for CountTo {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        for i in 0..self.count {
            println!("Count is {i}");
        }
        
        std::task::Poll::Ready(())
    }
}

async fn async_main(count: i32) {
    // count_to(count).await;
    let result = two_d10(count as u32).await;
    println!("Rolled Value = {result}");
}


fn roll_d10() -> RollD10Future {
    RollD10Future
}

// It is important to realize that the Future type does not have to be a struct
// async fn two_d10(modifier: u32) -> u32 {
//     let first_roll = roll_d10().await;
//     let second_roll = roll_d10().await;
//     first_roll + second_roll + modifier
// }

// equivalent Future implementation of the async version above
fn two_d10(modifier: u32) -> TwoD10 {
    TwoD10::Init { modifier }
}

///
/// This represent a roll that might still be in the process of rolling
/// and whose final value might not yet be known
struct RollD10Future;

impl Future for RollD10Future {
    type Output = u32;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut rng = rand::rng();
        Poll::Ready(rng.random_range(0..=10))
    }
}


enum TwoD10 {
    Init { modifier: u32 },
    FirstRoll { modifier: u32, fut: RollD10Future },
    SecondRoll { modifier: u32, first_roll: u32, fut: RollD10Future }
}

impl Future for TwoD10 {
    type Output = u32;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // I do not quite understand why a loop was needed here, 
        loop {
            match *self {
                TwoD10::Init { modifier } => {
                    let fut = roll_d10();
                    *self  = TwoD10::FirstRoll { modifier, fut }
                }
                TwoD10::FirstRoll { modifier, ref mut fut } => {
                    if let Poll::Ready(first_roll) = std::pin::Pin::new(fut).poll(cx) {
                        let fut = roll_d10();
                        *self  = TwoD10::SecondRoll { modifier, first_roll, fut }
                    } else {
                        return Poll::Pending;
                    }
                },
                TwoD10::SecondRoll { modifier, first_roll, ref mut fut } => {
                    if let Poll::Ready(second_roll) =  std::pin::Pin::new(fut).poll(cx) {
                        return Poll::Ready(modifier + first_roll + second_roll)
                    } else {
                        return Poll::Pending
                    }
                },
            }
        }
    }
}