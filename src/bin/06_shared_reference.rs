
fn main() {
    let a = 'A';
    let b = 'B';

    let mut r = &a;
    // here you might think that adding mut infront of the r variables creates a mutable link to a
    // but in reality, r is mutable and can be reassigned to another value, (
    // this reassignment does not change the value been referred to) but it's currently pointing to the value of a but it
    // does not take ownership

    // the opposite of referencing is dereferencing and this is achieved with *
    let ref_b = &b;
    println!("Original Value referenced: {}", *ref_b);

    // references can never be null in rust
    // references borrow the value which they refer to 
    // rust performs some automatic dereferencing when invoking methods

    let result = ref_b.is_ascii();
    println!("Result: {}", result);

    // *ref_b = 'X';
    println!("{}", *ref_b);

    // inorder to be able to mutate the value been reference we must use the mutable reference
    // this is also called exclusive reference(s) this is because only the reference can be used
    // to access this value, 
    let mut a = 'A';
    let d = &mut a;
    *d = 'a';

    println!("{a}");

    // there is a difference between these 2
    // let mut a = &g;
    // let a = &mut h;
}