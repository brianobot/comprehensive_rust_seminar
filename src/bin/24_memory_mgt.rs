fn main() {
    // creating a String puts fixed-sized metadata on the stack and dynamically sized data, the actual string on the heap
    let s1 = String::from("Hello");

    println!("String Capacity: {}", s1.capacity());
    println!("String Ptr: {:p}", s1.as_ptr());
    println!("Length: {}", s1.len());

    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("Capacity = {capacity}, ptr = {ptr:#x}, len = {len}")
    }
}
