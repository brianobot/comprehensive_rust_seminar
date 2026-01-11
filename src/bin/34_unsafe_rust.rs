


fn main() {
    // creating a pointer is safe, but dereferencing it requires unsafe
    // 
    let mut x = 10;
    let p1: *mut i32 = &raw mut x;
    let p2 = p1 as *const i32;
    
    
}