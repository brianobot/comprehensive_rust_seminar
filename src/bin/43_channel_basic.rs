use std::sync::mpsc;


fn main() {
    let (tx, rx) = mpsc::channel();
    let (bound_tx, bound_rx) = mpsc::sync_channel(3);
    
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    
    bound_tx.send(100).unwrap();
    bound_tx.send(300).unwrap();
    
    println!("Received From Unbounded Channel: {:?}", rx.recv());
    println!("Received From Unbounded Channel: {:?}", rx.recv());
    
    println!("Received from Bounded Channel: {:?}", bound_rx.recv());
    println!("Received from Bounded Channel: {:?}", bound_rx.recv());
    
    let tx2 = tx.clone();
    tx2.send(200).unwrap();
    
    let val = rx.recv().unwrap();
    println!("FInal Value Received: {val}");
}